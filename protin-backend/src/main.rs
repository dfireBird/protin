use std::env;

use dotenvy;

fn main() {
    dotenvy::from_path(env::var("ENV_FILE").unwrap_or("../.env".to_string()))
        .expect(".env file not found.");

    println!("Hello, world!");
}
