mod db;

use actix_web::web;
use anyhow::Context;
use rand::seq::IteratorRandom;

use crate::{bucket, models::Paste, AppState};

const KEY_LENGTH: u32 = 10;

const KEY_SPACE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

/// create a random file name
/// put the file data with file name into bucket
/// add an entry
pub async fn create_paste(app_data: AppState, file_data: &[u8]) -> anyhow::Result<Paste> {
    let key = generate_key(KEY_LENGTH);
    let file_path = uuid::Uuid::new_v4();
    bucket::put_file(&app_data.bucket, &file_path.to_string(), file_data).await?;
    web::block(move || {
        let mut conn = app_data
            .pool
            .get()
            .context("Couldn't get a database connection from pool")?;
        db::create_new_paste(&mut conn, key, file_path)
    })
    .await?
}

fn generate_key(key_length: u32) -> String {
    let mut key = String::new();

    let mut rng = rand::thread_rng();
    for _ in 0..key_length {
        let rand_char = KEY_SPACE
            .chars()
            .choose(&mut rng)
            .expect("It shouldn't panic, since iterator won't be empty");
        key.push(rand_char);
    }
    key
}
