//! adapters/dto/responses.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::entities::customer::Customer;
use crate::domain::entities::item::Item;
use crate::domain::entities::order::Order;

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
pub(crate) struct CustomerResponse {
    pub(crate) data: Customer,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct CustomersResponse {
    pub(crate) data: Vec<Customer>,
}
