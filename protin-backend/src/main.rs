use std::env;

use dotenvy;

fn main() {
    dotenvy::from_path(env::var("ENV_FILE").unwrap_or("../.env".to_string()))
        .expect(".env file not found.");

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "protin=debug,actix_web=info");
    }
    env_logger::init();

    protin::start_protin();
}
