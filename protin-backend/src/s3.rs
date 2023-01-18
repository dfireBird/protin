use std::env;

use anyhow::Context;
use aws_config;
use aws_sdk_s3::{
    config,
    model::{
        BucketLifecycleConfiguration, ExpirationStatus, LifecycleExpiration, LifecycleRule,
        LifecycleRuleFilter,
    },
    types::ByteStream,
    Endpoint, Region,
};

pub use aws_sdk_s3::Client;

const S3_BUCKET_NAME: &str = "protin-files";
const S3_BUCKET_LIFECYCLE_ID: &str = "protin-files-expiration-lifecycle";

pub async fn create_client() -> anyhow::Result<Client> {
    let env_config = aws_config::load_from_env().await;
    let config = config::Builder::from(&env_config)
        .region(Region::new(
            env::var("S3_REGION").context("S3_REGION environment variable must be set.")?,
        ))
        .endpoint_resolver(
            Endpoint::immutable(
                env::var("S3_ENDPOINT").context("S3_ENDPOINT environment variable must be set.")?,
            )
            .context("Invalid Enpoint resolver set in S3_ENDPOINT environment variable.")?,
        )
        .build();
    let client = Client::from_conf(config);
    create_bucket_if_not_exists(&client).await?;
    create_lifecycle_if_not_exists(&client).await?;
    Ok(client)
}

async fn create_bucket_if_not_exists(client: &Client) -> anyhow::Result<()> {
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
            .any(|n| n == S3_BUCKET_NAME);
        if is_bucket_listed {
            return Ok(());
        }
    }

    client
        .create_bucket()
        .bucket(S3_BUCKET_NAME)
        .send()
        .await
        .context(format!("Can't create the bucket {}", S3_BUCKET_NAME))?;

    Ok(())
}

pub async fn put_file(client: &Client, file_path: &str, file_data: Vec<u8>) -> anyhow::Result<()> {
    client
        .put_object()
        .bucket(S3_BUCKET_NAME)
        .key(file_path)
        .body(ByteStream::from(file_data))
        .send()
        .await
        .context(format!(
            "Can't put object in the bucket: {}",
            S3_BUCKET_NAME
        ))?;
    Ok(())
}

pub async fn get_file(client: &Client, file_path: &str) -> anyhow::Result<Vec<u8>> {
    let resp = client
        .get_object()
        .bucket(S3_BUCKET_NAME)
        .key(file_path)
        .send()
        .await
        .context(format!(
            "Can't put object in the bucket: {}",
            S3_BUCKET_NAME
        ))?;
    Ok(resp
        .body
        .collect()
        .await
        .context("Error while collecting the ByteStream")?
        .to_vec())
}

async fn create_lifecycle_if_not_exists(client: &Client) -> anyhow::Result<()> {
    let resp = client
        .get_bucket_lifecycle_configuration()
        .bucket(S3_BUCKET_NAME)
        .send()
        .await
        .context("Error in list buckets lifecycle configuration request. ")?;
    let rules = resp.rules();

    if let Some(rules_slice) = rules {
        let is_rule_listed = rules_slice
            .iter()
            .map(|r| r.id().unwrap_or(""))
            .any(|id| id == S3_BUCKET_LIFECYCLE_ID);
        if is_rule_listed {
            return Ok(());
        }
    }
    let bucket_lifecycle_rule = BucketLifecycleConfiguration::builder()
        .rules(
            LifecycleRule::builder()
                .status(ExpirationStatus::Enabled)
                .filter(LifecycleRuleFilter::Prefix("".to_string()))
                .id(S3_BUCKET_LIFECYCLE_ID)
                .expiration(LifecycleExpiration::builder().days(1).build())
                .build(),
        )
        .build();

    client
        .put_bucket_lifecycle_configuration()
        .bucket(S3_BUCKET_NAME)
        .lifecycle_configuration(bucket_lifecycle_rule)
        .send()
        .await
        .context(format!(
            "Can't set lifecycle rules for the bucket: {}",
            S3_BUCKET_NAME
        ))?;
    Ok(())
}
