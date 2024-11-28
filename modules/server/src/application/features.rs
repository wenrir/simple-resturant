//! application/features.rs
//! Application features (usecases)
//! TODO use these inside

use anyhow::Result;
use diesel::prelude::*;
use diesel::PgConnection;
use diesel::{QueryDsl, SelectableHelper};

use crate::domain::entities::customer::Customer;
use crate::domain::entities::customers::total;

/// Get all active customers, i.e. customers which has not paid yet.
/// (Could obviously be handled with another type of flag.)
pub(crate) fn get_all_active_customers(conn: &mut PgConnection) -> Result<Vec<Customer>> {
    use crate::domain::entities::customers;
    let all = customers::table
        .select(Customer::as_select())
        .filter(total.eq(0))
        .load(conn)?;
    Ok(all)
}

/// Get customer from id
pub(crate) fn get_customer(conn: &mut PgConnection, cid: &i32) -> Result<Vec<Customer>> {
    use crate::domain::entities::customers;
    let customer = customers::table
        .select(Customer::as_select())
        .filter(crate::domain::entities::customers::id.eq(cid))
        .load(conn)?;
    Ok(customer)
}
