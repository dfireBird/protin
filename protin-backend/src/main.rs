use std::env;

use anyhow::Context;
use dotenvy;

use protin::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::from_path(env::var("ENV_FILE").unwrap_or("../.env".to_string()))
        .context(".env file not found.")?;

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "protin=debug,actix_web=info");
    }
    env_logger::init();

    let config = Config::from_env()?;

    protin::start_protin(config).await
}
