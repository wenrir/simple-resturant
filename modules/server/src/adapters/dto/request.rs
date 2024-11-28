//! adapters/dto/request.rs

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct OrderCreateRequest {
    pub(crate) item_id: i32,
    pub(crate) customer_id: i32,
    pub(crate) quantity: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct ItemCreateRequest {
    pub(crate) description: String,
    pub(crate) estimated_minutes: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct CustomerGetRequest {
    pub(crate) id: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct CustomerCreateRequest {
    pub(crate) table_number: i32,
}
