use anyhow::Context;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::config::Config;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn create_db_pool(config: &Config) -> anyhow::Result<DbPool> {
    let manager = ConnectionManager::new(config.database_url());
    Pool::builder()
        .build(manager)
        .context("Can't create a db connection pool")
}

pub fn run_migrations(conn: &mut PgConnection) -> anyhow::Result<()> {
    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(..) => Ok(()),
        Err(e) => Err(anyhow::anyhow!(
            "Error while running pending migrations: {:?}",
            e
        )),
    }
}
