use std::{io, net::Ipv6Addr};

use actix_cors::Cors;
use actix_governor::{GlobalKeyExtractor, Governor, GovernorConfigBuilder};
use actix_multipart::form::MultipartFormConfig;
use actix_web::{App, HttpServer, middleware::Logger, web};
use anyhow::Context;
use log::info;
use middlewares::ratelimiter::{RealIpKeyExtractor, get_ns_per_request};

pub use crate::config::Config;

mod config;
mod db;
mod middlewares;
mod models;
mod paste;
mod routes;
mod s3;
mod schema;

#[derive(Clone, Debug)]
pub struct AppState {
    pool: db::DbPool,
    s3_client: s3::Client,
    s3_bucket_name: String,
    s3_total_size_limit: usize,
}

pub async fn start_protin(config: Config) -> anyhow::Result<()> {
    let pool = db::create_db_pool(&config)?;
    info!("Connection Pool is created");

    {
        let mut conn = pool
            .get()
            .context("Couldn't get a database connection from pool for migrations")?;

        db::run_migrations(&mut conn).context("Couldn't run db migrations")?;
    }

    let client = s3::create_client(&config).await?;
    info!("S3 Client is created");

    create_server(pool, client, &config)
        .await
        .context("Web server can't be created.")?;
    Ok(())
}

async fn create_server(pool: db::DbPool, s3_client: s3::Client, config: &Config) -> io::Result<()> {
    let file_size_limit = config.s3_file_size_limit();
    let app_state = AppState {
        pool,
        s3_client,
        s3_bucket_name: config.s3_bucket_name(),
        s3_total_size_limit: config.s3_total_size_limit(),
    };

    let reverse_proxy_ip = config.reverse_proxy_ip();
    let ip_limiter_config = GovernorConfigBuilder::default()
        .const_burst_size(config.ip_limit_per_second())
        .const_nanoseconds_per_request(get_ns_per_request(config.ip_limit_per_second()))
        .key_extractor(RealIpKeyExtractor { reverse_proxy_ip })
        .use_headers()
        .finish()
        .unwrap();
    let global_limiter_config = GovernorConfigBuilder::default()
        .const_burst_size(config.global_limit_per_second())
        .const_nanoseconds_per_request(get_ns_per_request(config.global_limit_per_second()))
        .key_extractor(GlobalKeyExtractor)
        .finish()
        .unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allow_any_header();

        App::new()
            .app_data(MultipartFormConfig::default().total_limit(file_size_limit))
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Governor::new(&ip_limiter_config))
            .wrap(Governor::new(&global_limiter_config))
            .wrap(Logger::default())
            .wrap(cors)
            .service(web::scope("/api").configure(routes::pastes_config))
    })
    .bind((Ipv6Addr::UNSPECIFIED, config.web_port()))?
    .run()
    .await
}
