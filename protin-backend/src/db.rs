use anyhow::Context;
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

use crate::config::Config;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn create_db_pool(config: &Config) -> anyhow::Result<DbPool> {
    let _ = diesel::connection::set_default_instrumentation(|| {
        use diesel::connection::InstrumentationEvent;

        let instrumentation = |event: InstrumentationEvent| {
            if let InstrumentationEvent::FinishQuery { query, .. } = event {
                log::trace!(target: "db_events", "Executed query: {query}");
            }
        };
        Some(Box::new(instrumentation))
    });

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
