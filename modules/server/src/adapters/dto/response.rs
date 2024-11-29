//! adapters/dto/responses.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::entities::item::Item;
use crate::domain::entities::order::Order;
use crate::domain::entities::table::Table;

// TODO move these to a shared lib.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct OrderResponse {
    pub(crate) data: Vec<Order>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct ItemResponse {
    pub(crate) data: Item,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct ItemsResponse {
    pub(crate) data: Vec<Item>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct TableResponse {
    pub(crate) data: Table,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct TablesResponse {
    pub(crate) data: Vec<Table>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct CheckoutResponse {
    pub(crate) data: i32,
}
