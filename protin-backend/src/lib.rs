use std::{env, io};

use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Context;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use log::info;
use s3::Bucket;

mod bucket;
mod models;
mod paste;
mod schema;

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Clone, Debug)]
pub struct AppState {
    pool: DbPool,
    bucket: Bucket,
}

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

pub fn start_protin() -> anyhow::Result<()> {
    let manager = ConnectionManager::new(get_database_url()?);
    let pool = Pool::builder()
        .build(manager)
        .context("Can't create a db connection pool")?;
    info!("Connection Pool is created");

    let bucket = bucket::create_bucket()?;
    info!("R2 Bucket object is created");

    create_server(pool, bucket).context("Web server can't be created.")?;
    Ok(())
}

#[actix_web::main]
async fn create_server(pool: DbPool, bucket: Bucket) -> io::Result<()> {
    let app_state = AppState { pool, bucket };
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .service(hello_world)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

fn get_database_url() -> anyhow::Result<String> {
    if let Ok(url) = env::var("DATABASE_URL") {
        Ok(url)
    } else {
        let user = env::var("POSTGRES_USER").context(
            "POSTGRES_USER environment variable must be set if DATABASE_URL is not set.",
        )?;
        let password = env::var("POSTGRES_PASSWORD").context(
            "POSTGRES_PASSWORD environment variable must be set if DATABASE_URL is not set.",
        )?;
        let db = env::var("POSTGRES_DB")
            .context("POSTGRES_DB environment variable must be set if DATABASE_URL is not set.")?;
        let host = env::var("POSTGRES_HOST").unwrap_or("localhost".to_string());

        Ok(format!("postgres://{user}:{password}:{host}/{db}"))
    }
}
