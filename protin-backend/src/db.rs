use anyhow::Context;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

use crate::config::Config;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_db_pool(config: &Config) -> anyhow::Result<DbPool> {
    let manager = ConnectionManager::new(config.database_url());
    Pool::builder()
        .build(manager)
        .context("Can't create a db connection pool")
}
