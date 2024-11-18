//! Customer
use super::customers;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
#[derive(Identifiable, Selectable, Queryable, Debug, Deserialize, Serialize, PartialEq)]
#[diesel(table_name = customers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)] // TODO
pub(crate) struct Customer {
    #[serde(skip_serializing)]
    pub(crate) id: i32,
    pub(crate) checked_in_time: SystemTime,
    pub(crate) total: i32,
}

#[derive(Insertable)]
#[diesel(table_name = customers)]
pub struct NewCustomer<'a> {
    pub(crate) checked_in_time: &'a SystemTime,
    pub(crate) total: &'a i32,
}
