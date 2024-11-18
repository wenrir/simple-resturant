//! infrastructure/db.rs

use crate::adapters::db_connect;
use anyhow::Result;
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

#[allow(unused)]
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
