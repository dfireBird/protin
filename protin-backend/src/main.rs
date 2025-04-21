use env_logger::Env;

use protin::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if dotenvy::from_path("../.env".to_string()).is_err() {
        eprintln!(".env file not found. Environment variables are assumed to be set.")
    };

    let logger_env = Env::default().default_filter_or("protin=debug,actix_web=info");
    env_logger::init_from_env(logger_env);

    let config = Config::from_env()?;

    protin::start_protin(config).await
}
