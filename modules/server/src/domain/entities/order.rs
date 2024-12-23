//! Order

use super::{item::Item, orders, table::Table};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Identifiable, Selectable, Queryable, Associations, Debug, Deserialize, Serialize, ToSchema,
)]
#[diesel(table_name = orders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Item))]
#[diesel(belongs_to(Table))]
#[diesel(primary_key(table_id, item_id))]
pub(crate) struct Order {
    pub(crate) id: i32,
    pub(crate) published_at: String,
    // This should probably be a list instead.
    // then I could add orders as:
    // {
    //  "table_id": 1,
    //  "items": [
    //    { "item_id": 101, "quantity": 2 },
    //    { "item_id": 102, "quantity": 5 }
    //  ]
    //}
    pub(crate) quantity: i32,
    pub(crate) item_id: i32,
    #[serde(skip_serializing)]
    pub(crate) table_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = orders)]
pub struct NewOrder<'a> {
    pub(crate) item_id: &'a i32,
    pub(crate) table_id: &'a i32,
    pub(crate) published_at: &'a String,
    pub(crate) quantity: &'a i32,
}
