use std::env;

use anyhow::Context;
use dotenvy;

use protin::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::from_path(env::var("ENV_FILE").unwrap_or("../.env".to_string()))
        .context(".env file not found.")?;

    if env::var("RUST_LOG").ok().is_none() {
        unsafe {
            /*
             * The function is unsafe only on multi-threaded programs on OS other than Windows.
             * Tho this the web server is multi-threaded program, at this point the threads are
             * spawned yet. So I think it's mostly safe to use.
             */
            // FIXME: find a different way to do this
            env::set_var("RUST_LOG", "protin=debug,actix_web=info");
        }
    }
    env_logger::init();

    let config = Config::from_env()?;

    protin::start_protin(config).await
}
