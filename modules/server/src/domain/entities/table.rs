//! Table
use super::tables;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(
    Identifiable, Selectable, Queryable, Debug, Deserialize, Serialize, PartialEq, ToSchema,
)]
#[diesel(table_name = tables)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct Table {
    #[serde(skip_serializing)]
    pub(crate) id: i32,
    pub(crate) checked_in_time: String,
    pub(crate) table_number: i32,
    pub(crate) total: i32,
}

#[derive(Insertable)]
#[diesel(table_name = tables)]
pub struct NewTable<'a> {
    pub(crate) checked_in_time: &'a String,
    pub(crate) total: &'a i32,
    pub(crate) table_number: &'a i32,
}
