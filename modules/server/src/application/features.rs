//! application/features.rs
//! Application features (usecases)
//! TODO use these inside

use anyhow::Result;
use diesel::prelude::*;
use diesel::PgConnection;
use diesel::{QueryDsl, SelectableHelper};

use crate::domain::entities::customer::Customer;
use crate::domain::entities::customers::total;

use super::repo::OrderRepository;

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

#[allow(dead_code)]
pub(crate) struct OrderFeature<'a> {
    table_number: &'a i32,
    order_repository: &'a (dyn OrderRepository + Send + Sync),
}

#[allow(dead_code)]
impl<'a> OrderFeature<'a> {
    pub(crate) fn new(table: &'a i32, repository: &'a (dyn OrderRepository + Send + Sync)) -> Self {
        OrderFeature {
            table_number: table,
            order_repository: repository,
        }
    }
}

//#[async_trait(?Send)]
//impl<'a> AbstractFeature<Order> for OrderFeature<'a> {
//    async fn execute(&self) -> ServerResult<Order> {
//        let order = self.order_repository.create(&NewOrder {
//            table_number: self.table_number,
//        })?;
//        Ok(order)
//    }
//}
