use anyhow::Context;
use aws_config::{self, BehaviorVersion, Region};
use aws_sdk_s3::{
    config,
    primitives::ByteStream,
    types::{
        BucketLifecycleConfiguration, ExpirationStatus, LifecycleExpiration, LifecycleRule,
        LifecycleRuleFilter,
    },
};

pub use aws_sdk_s3::Client;

use crate::config::Config;

pub async fn create_client(app_config: &Config) -> anyhow::Result<Client> {
    let env_config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(app_config.s3_region()))
        .endpoint_url(app_config.s3_endpoint())
        .load()
        .await;
    let config = config::Builder::from(&env_config)
        .force_path_style(true)
        .build();
    let client = Client::from_conf(config);
    create_bucket_if_not_exists(&client, &app_config).await?;
    create_lifecycle_if_not_exists(&client, app_config).await?;
    Ok(client)
}

async fn create_bucket_if_not_exists(client: &Client, app_config: &Config) -> anyhow::Result<()> {
    let bucket_resp = client
        .list_buckets()
        .send()
        .await
        .context("List buckets request can't be sent")?;
    let buckets = bucket_resp.buckets();

    if !buckets.is_empty() {
        let is_bucket_listed = buckets
            .iter()
            .map(|b| b.name().unwrap_or(""))
            .any(|n| n == app_config.s3_bucket_name());
        if is_bucket_listed {
            return Ok(());
        }
    }

    client
        .create_bucket()
        .bucket(app_config.s3_bucket_name())
        .send()
        .await
        .context(format!(
            "Can't create the bucket {}",
            app_config.s3_bucket_name()
        ))?;

    Ok(())
}

pub async fn put_file(
    client: &Client,
    s3_bucket_name: &str,
    file_path: &str,
    file_data: Vec<u8>,
) -> anyhow::Result<()> {
    client
        .put_object()
        .bucket(s3_bucket_name)
        .key(file_path)
        .body(ByteStream::from(file_data))
        .send()
        .await
        .context(format!(
            "Can't put object in the bucket: {}",
            s3_bucket_name
        ))?;
    Ok(())
}

pub async fn get_file(
    client: &Client,
    s3_bucket_name: &str,
    file_path: &str,
) -> anyhow::Result<Vec<u8>> {
    let resp = client
        .get_object()
        .bucket(s3_bucket_name)
        .key(file_path)
        .send()
        .await
        .context(format!(
            "Can't get object in the bucket: {}",
            s3_bucket_name
        ))?;
    Ok(resp
        .body
        .collect()
        .await
        .context("Error while collecting the ByteStream")?
        .to_vec())
}

async fn create_lifecycle_if_not_exists(
    client: &Client,
    app_config: &Config,
) -> anyhow::Result<()> {
    let resp = client
        .get_bucket_lifecycle_configuration()
        .bucket(app_config.s3_bucket_name())
        .send()
        .await;
    // NOTE: MinIO returns an error if there are no lifecycle present, so handling it as no rules
    let rules = resp.map(|r| r.rules).unwrap_or_default();

    if let Some(rules) = rules {
        let is_rule_listed = rules
            .iter()
            .map(|r| r.id().unwrap_or(""))
            .any(|id| id == app_config.s3_bucket_lifcycle_id());
        if is_rule_listed {
            return Ok(());
        }
    }
    let bucket_lifecycle_rule = BucketLifecycleConfiguration::builder()
        .rules(
            LifecycleRule::builder()
                .status(ExpirationStatus::Enabled)
                .filter(LifecycleRuleFilter::Prefix("".to_string()))
                .id(app_config.s3_bucket_lifcycle_id())
                .expiration(
                    LifecycleExpiration::builder()
                        .days(app_config.s3_bucket_expiration())
                        .build(),
                )
                .build()
                .context(format!("Can't build lifecycle rule"))?,
        )
        .build()
        .context(format!(
            "Can't build Bucket Lifecycle Configuration: {}",
            app_config.s3_bucket_name()
        ))?;

    client
        .put_bucket_lifecycle_configuration()
        .bucket(app_config.s3_bucket_name())
        .lifecycle_configuration(bucket_lifecycle_rule)
        .send()
        .await
        .context(format!(
            "Can't set lifecycle rules for the bucket: {}",
            app_config.s3_bucket_name()
        ))?;
    Ok(())
}
