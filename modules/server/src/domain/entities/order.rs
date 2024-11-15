//! Order
use std::time::SystemTime;

use super::{items, orders};
use diesel::prelude::*;
#[allow(unused)] // remove me
#[derive(Queryable, Selectable)]
#[diesel(table_name = orders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct Order {
    pub(crate) id: i32,
    pub(crate) published_at: SystemTime,
    pub(crate) table_number: i32,
}

#[allow(unused)] // Remove me
#[derive(Queryable, Selectable)]
#[diesel(table_name = items)]
#[diesel(belongs_to(Order))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct Item {
    pub(crate) id: i32,
    pub(crate) quantity: i32,
    pub(crate) description: String,
}

#[derive(Insertable)]
#[diesel(table_name = orders)]
pub struct NewOrder<'a> {
    pub(crate) table_number: &'a i32,
}
