//! infrastructure/db.rs

use anyhow::Result;
use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env::var;

#[allow(unused)]
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub(crate) fn connect() -> Result<PgConnection> {
    let database_url = var("DATABASE_URL").expect("Database URL needs to be set!");

    let conn = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    Ok(conn)
}

#[allow(unused)]
///Establish database connection, load `.env` for default DB_URL (or in terms of docker read env var.)
pub(crate) fn migrate() -> Result<()> {
    let run_migration = |conn: &mut PgConnection| {
        // Migrate on database connection!
        conn.run_pending_migrations(MIGRATIONS)
            .expect("Unable to migrate!");
    };
    let mut conn = connect()?;
    run_migration(&mut conn);
    Ok(())
}
