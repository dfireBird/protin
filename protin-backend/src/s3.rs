use anyhow::Context;
use aws_config::{self, BehaviorVersion, Region};
use aws_sdk_s3::{
    config,
    primitives::ByteStream,
    types::{Delete, ObjectIdentifier},
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
    create_bucket_if_not_exists(&client, app_config).await?;
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
    file_key: &str,
    file_data: Vec<u8>,
) -> anyhow::Result<()> {
    client
        .put_object()
        .bucket(s3_bucket_name)
        .key(file_key)
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
    file_key: &str,
) -> anyhow::Result<Vec<u8>> {
    let resp = client
        .get_object()
        .bucket(s3_bucket_name)
        .key(file_key)
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

pub async fn delete_files(
    client: &Client,
    s3_bucket_name: &str,
    file_keys: Vec<String>,
) -> anyhow::Result<(Vec<String>, Option<anyhow::Error>)> {
    let obj_ids_iter = file_keys
        .iter()
        .map(|k| (k, ObjectIdentifier::builder().key(k).build()));

    let mut ok_ids = Vec::new();
    let mut err_ids = Vec::new();
    obj_ids_iter.for_each(|(k, o)| match o {
        Ok(obj_id) => ok_ids.push(obj_id),
        Err(err) => err_ids.push(anyhow::anyhow!(
            "Error while building delete request for file {}: {}",
            k,
            err
        )),
    });

    if err_ids.len() > 0 {
        let err_message: Vec<_> = err_ids.iter().map(|err| format!("{err}")).collect();
        let err_message = err_message.join("\n");
        return Err(anyhow::anyhow!("{}", err_message));
    }

    let delete = Delete::builder()
        .set_objects(Some(ok_ids))
        .build()
        .context("while building delete request")?;

    let resp = client
        .delete_objects()
        .bucket(s3_bucket_name)
        .delete(delete)
        .send()
        .await
        .context("while sending the DeleteObjects request")?;

    let deleted_ids = resp
        .deleted()
        .iter()
        .filter_map(|obj| obj.key())
        .map(|s| s.to_string())
        .collect();

    let mut errors = None;
    if let Some(errs) = resp.errors {
        let err: Vec<_> = errs
            .into_iter()
            .map(|e| {
                format!(
                    "Error while deleting file {}: {}({})",
                    e.key().unwrap_or_default(),
                    e.message().unwrap_or_default(),
                    e.code().unwrap_or_default(),
                )
            })
            .collect();
        errors = Some(anyhow::anyhow!("{}", err.join("\n")));
    }

    Ok((deleted_ids, errors))
}
