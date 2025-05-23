mod db;

use actix_web::web;
use anyhow::Context;
use rand::seq::IteratorRandom;

use crate::{models::Paste, s3, AppState};

const KEY_LENGTH: u32 = 10;

const KEY_SPACE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

/// create a random file name
/// put the file data with file name into bucket
/// add an entry
pub async fn create_paste(
    app_data: web::Data<AppState>,
    file_data: &[u8],
) -> anyhow::Result<Paste> {
    let key = generate_key(KEY_LENGTH);
    let file_path = uuid::Uuid::new_v4();
    s3::put_file(
        &app_data.s3_client,
        &app_data.s3_bucket_name,
        &file_path.to_string(),
        file_data.to_vec(),
    )
    .await?;
    web::block(move || {
        let mut conn = app_data
            .pool
            .get()
            .context("Couldn't get a database connection from pool")?;
        db::create_new_paste(&mut conn, key, file_path)
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
        s3::get_file(
            &data.s3_client,
            &data.s3_bucket_name,
            &paste.file_path.to_string(),
        )
        .await
        .map(Some)
    } else {
        Ok(None)
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
