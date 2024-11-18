//! adapters/dto/responses.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::entities::customer::Customer;
use crate::domain::entities::item::Item;
use crate::domain::entities::order::Order;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct OrderResponse {
    pub(crate) order: Vec<Order>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct ItemResponse {
    pub(crate) item: Item,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct CustomerResponse {
    pub(crate) customer: Customer,
}
