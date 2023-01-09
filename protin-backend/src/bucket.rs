use std::env;

use anyhow::Context;
use aws_config;
use aws_sdk_s3::{config, types::ByteStream, Endpoint, Region};

pub use aws_sdk_s3::Client;

const S3_BUCKET_NAME: &str = "protin-files";

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
    Ok(client)
}

async fn create_bucket_if_not_exists(client: &Client) -> anyhow::Result<()> {
    let bucket_resp = client
        .list_buckets()
        .send()
        .await
        .context("List buckets request can't be sent")?;
    let buckets = bucket_resp.buckets();

    // check if bucket is there
    if let Some(buckets_slice) = buckets {
        let is_bucket_listed = buckets_slice
            .iter()
            .map(|b| b.name().unwrap_or(""))
            .any(|n| n == S3_BUCKET_NAME);
        if is_bucket_listed {
            return Ok(());
        }
    }

    // if not create the bucket
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
