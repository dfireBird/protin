use std::env;

use anyhow::{anyhow, Context, Result};
use s3::{creds::Credentials, Bucket, BucketConfiguration};

pub async fn create_bucket() -> anyhow::Result<Bucket> {
    let response = Bucket::create(
        "protin-files",
        s3::Region::Custom {
            region: env::var("S3_REGION").context("S3_REGION environment variable must be set.")?,
            endpoint: env::var("S3_ENDPOINT")
                .context("S3_ENDPOINT environment variable must be set.")?,
        },
        Credentials::default().context("Invalid Credentials")?,
        BucketConfiguration::default(),
    )
    .await;
    match response {
        Ok(ok_resp) => Ok(ok_resp.bucket),
        Err(_) => Ok(Bucket::new(
            "protin-files",
            s3::Region::Custom {
                region: env::var("S3_REGION")
                    .context("S3_REGION environment variable must be set.")?,
                endpoint: env::var("S3_ENDPOINT")
                    .context("S3_ENDPOINT environment variable must be set.")?,
            },
            Credentials::default().context("Invalid Credentials")?,
        )
        .context("Can't instantiate the bucket object.")?),
    }
}

pub async fn put_file(bucket: &Bucket, file_path: &str, file_data: &[u8]) -> Result<()> {
    let response = bucket
        .put_object(file_path, file_data)
        .await
        .with_context(|| format!("Can't put object in the bucket: {}", bucket.name))?;

    // TODO: create a error object instead of using anyhow! errors
    if response.status_code() != 200 {
        return Err(anyhow!(format!(
            "Status Code (from bucket) recieved: {}",
            response.status_code()
        )));
    }

    Ok(())
}

pub async fn get_file(bucket: &Bucket, file_path: &str) -> Result<Vec<u8>> {
    let response = bucket
        .get_object(file_path)
        .await
        .with_context(|| format!("Can't put object in the bucket: {}", bucket.name))?;

    // TODO: create a error object instead of using anyhow! errors
    if response.status_code() != 200 {
        return Err(anyhow!(format!(
            "Status Code (from bucket) recieved: {}",
            response.status_code()
        )));
    }

    Ok(response.bytes().to_vec())
}
