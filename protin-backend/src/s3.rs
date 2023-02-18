use anyhow::Context;
use aws_config;
use aws_sdk_s3::{
    config,
    model::{
        BucketLifecycleConfiguration, ExpirationStatus, LifecycleExpiration, LifecycleRule,
        LifecycleRuleFilter,
    },
    types::ByteStream,
    Region,
};

pub use aws_sdk_s3::Client;

use crate::config::Config;

pub async fn create_client(app_config: &Config) -> anyhow::Result<Client> {
    let env_config = aws_config::load_from_env().await;
    let config = config::Builder::from(&env_config)
        .region(Region::new(app_config.s3_region()))
        .endpoint_url(app_config.s3_endpoint())
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

    if let Some(buckets_slice) = buckets {
        let is_bucket_listed = buckets_slice
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
        .await
        .context(format!(
            "List buckets lifecycle configuration request for bucket: {}.",
            app_config.s3_bucket_name()
        ))?;
    let rules = resp.rules();

    if let Some(rules_slice) = rules {
        let is_rule_listed = rules_slice
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
                .build(),
        )
        .build();

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
