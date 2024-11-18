//! Order
use std::time::SystemTime;

use super::{customer::Customer, item::Item, orders};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Identifiable, Selectable, Queryable, Associations, Debug, Deserialize, Serialize, ToSchema,
)]
#[diesel(table_name = orders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Item))]
#[diesel(belongs_to(Customer))]
#[allow(dead_code)] // TODO
#[diesel(primary_key(customer_id, item_id))]
pub(crate) struct Order {
    pub(crate) id: i32,
    pub(crate) published_at: String,
    pub(crate) table_number: i32, // TODO: This should probably be moved to Customer
    pub(crate) quantity: i32,
    #[serde(skip_serializing)]
    pub(crate) item_id: i32,
    #[serde(skip_serializing)]
    pub(crate) customer_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = orders)]
pub struct NewOrder<'a> {
    pub(crate) table_number: &'a i32,
    pub(crate) item_id: &'a i32,
    pub(crate) customer_id: &'a i32,
    pub(crate) published_at: &'a String,
    pub(crate) quantity: &'a i32,
}
