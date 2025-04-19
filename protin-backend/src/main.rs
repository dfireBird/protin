use std::env;

use anyhow::Context;
use env_logger::Env;

use protin::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::from_path(env::var("ENV_FILE").unwrap_or("../.env".to_string()))
        .context(".env file not found.")?;

    let logger_env = Env::default().default_filter_or("protin=debug,actix_web=info");
    env_logger::init_from_env(logger_env);

    let config = Config::from_env()?;

    protin::start_protin(config).await
}
