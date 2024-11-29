//! infrastructure/db.rs

use crate::adapters::db_connect;
use anyhow::Result;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env::var;

/// Get connection pool to db
pub(crate) fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = var("DATABASE_URL").expect("Database URL needs to be set!");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .max_size(20)
        .build(manager)
        .expect("Could not build connection pool")
}

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[allow(unused)]
///Establish database connection, load `.env` for default DB_URL (or in terms of docker read env var.)
pub(crate) fn migrate() -> Result<()> {
    let run_migration = |conn: &mut PgConnection| {
        // Migrate on database connection!
        conn.run_pending_migrations(MIGRATIONS)
            .expect("Unable to migrate!");
    };
    let mut conn = db_connect()?;
    run_migration(&mut conn);
    Ok(())
}
