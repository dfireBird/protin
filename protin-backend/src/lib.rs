use std::io;

use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Context;
use log::info;
use s3::Bucket;

mod bucket;
mod db;
mod models;
mod paste;
mod routes;
mod schema;

#[derive(Clone, Debug)]
pub struct AppState {
    pool: db::DbPool,
    bucket: Bucket,
}

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[tokio::main]
pub async fn start_protin() -> anyhow::Result<()> {
    let pool = db::create_db_pool()?;
    info!("Connection Pool is created");

    let bucket = bucket::create_bucket().await?;
    info!("S3 Bucket object is created");

    create_server(pool, bucket)
        .await
        .context("Web server can't be created.")?;
    Ok(())
}

async fn create_server(pool: db::DbPool, bucket: Bucket) -> io::Result<()> {
    let app_state = AppState { pool, bucket };
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .configure(routes::pastes_config)
            .service(hello_world)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
