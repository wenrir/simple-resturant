//! application/features.rs
//! Application features (usecases)
//! TODO use these inside

use anyhow::Result;
use diesel::prelude::*;
use diesel::PgConnection;
use diesel::{QueryDsl, SelectableHelper};

use crate::domain::entities::table::Table;
use crate::domain::entities::tables::total;

/// Get all active tables, i.e. tables which has not paid yet.
/// (Could obviously be handled with another type of flag.)
pub(crate) fn get_all_active_tables(conn: &mut PgConnection) -> Result<Vec<Table>> {
    use crate::domain::entities::tables;
    let all = tables::table
        .select(Table::as_select())
        .filter(total.eq(0))
        .load(conn)?;
    Ok(all)
}

/// Get table from id
pub(crate) fn get_table(conn: &mut PgConnection, cid: &i32) -> Result<Vec<Table>> {
    use crate::domain::entities::tables;
    let table = tables::table
        .select(Table::as_select())
        .filter(crate::domain::entities::tables::id.eq(cid))
        .load(conn)?;
    Ok(table)
}
