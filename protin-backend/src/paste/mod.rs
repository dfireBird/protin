mod db;

use actix_web::web;
use anyhow::Context;
use rand::seq::IteratorRandom;

use crate::{AppState, models::Paste, s3};

const KEY_LENGTH: u32 = 20;

const KEY_SPACE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_";

/// create a random file name
/// put the file data with file name into bucket
/// add an entry
pub async fn create_paste(
    app_data: web::Data<AppState>,
    file_data: &[u8],
    expires_at: Option<u64>,
) -> anyhow::Result<Paste> {
    let key = generate_key(KEY_LENGTH);
    s3::put_file(
        &app_data.s3_client,
        &app_data.s3_bucket_name,
        &key,
        file_data.to_vec(),
    )
    .await?;
    web::block(move || {
        let mut conn = app_data
            .pool
            .get()
            .context("Couldn't get a database connection from pool")?;
        db::create_new_paste(&mut conn, key, expires_at)
    })
    .await?
}

pub async fn get_paste(
    app_data: web::Data<AppState>,
    id: String,
) -> anyhow::Result<Option<Vec<u8>>> {
    let data = app_data.clone();
    let paste = web::block(move || {
        let mut conn = app_data
            .pool
            .get()
            .context("Couldn't get a database connection from pool")?;
        db::get_paste(&mut conn, id)
    })
    .await??;

    if let Some(paste) = paste {
        s3::get_file(&data.s3_client, &data.s3_bucket_name, &paste.id)
            .await
            .map(Some)
    } else {
        Ok(None)
    }
}

pub async fn cleanup_expired_paste(app_data: web::Data<AppState>) -> anyhow::Result<()> {
    let data = app_data.clone();
    let expired_ids = web::block(move || {
        let mut conn = app_data
            .pool
            .get()
            .context("Couldn't get a database connection from pool")?;
        db::get_expired_paste_ids(&mut conn)
    })
    .await??;

    let (deleted_ids, error) =
        s3::delete_files(&data.s3_client, &data.s3_bucket_name, expired_ids).await?;

    let deleted_size = deleted_ids.len();
    let updated_size = web::block(move || {
        let mut conn = data
            .pool
            .get()
            .context("Couldn't get a database connection from pool")?;
        db::set_deleted_for_ids(&mut conn, deleted_ids)
    })
    .await??;

    if let Some(error) = error {
        Err(error)
    } else if updated_size != deleted_size {
        Err(anyhow::anyhow!(
            "Update of deletion in table is partial {}/{}",
            updated_size,
            deleted_size
        ))
    } else {
        Ok(())
    }
}

fn generate_key(key_length: u32) -> String {
    let mut key = String::new();

    let mut rng = rand::rng();
    for _ in 0..key_length {
        let rand_char = KEY_SPACE
            .chars()
            .choose(&mut rng)
            .expect("It shouldn't panic, since iterator won't be empty");
        key.push(rand_char);
    }
    key
}
