//! Item
use super::items;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Selectable, Queryable, Debug, Deserialize, Serialize, PartialEq)]
#[diesel(table_name = items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct Item {
    #[allow(unused)]
    #[serde(skip_serializing)]
    pub(crate) id: i32,
    pub(crate) estimated_minutes: i32,
    pub(crate) description: String,
}

#[derive(Insertable)]
#[diesel(table_name = items)]
pub struct NewItem<'a> {
    pub(crate) description: &'a String,
    pub(crate) estimated_minutes: &'a i32,
}
