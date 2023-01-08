use std::env;

use anyhow::Context;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_db_pool() -> anyhow::Result<DbPool> {
    let manager = ConnectionManager::new(get_database_url()?);
    Pool::builder()
        .build(manager)
        .context("Can't create a db connection pool")
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
