use std::io;

use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Context;
use log::info;

mod db;
mod models;
mod paste;
mod routes;
mod s3;
mod schema;

#[derive(Clone, Debug)]
pub struct AppState {
    pool: db::DbPool,
    s3_client: s3::Client,
}

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[tokio::main]
pub async fn start_protin() -> anyhow::Result<()> {
    let pool = db::create_db_pool()?;
    info!("Connection Pool is created");

    let client = s3::create_client().await?;
    info!("S3 Client is created");

    create_server(pool, client)
        .await
        .context("Web server can't be created.")?;
    Ok(())
}

async fn create_server(pool: db::DbPool, s3_client: s3::Client) -> io::Result<()> {
    let app_state = AppState { pool, s3_client };
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
