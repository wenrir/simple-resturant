//! adapters/dto/request.rs

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct OrderCreateRequest {
    pub(crate) item_id: i32,
    pub(crate) table_id: i32,
    pub(crate) quantity: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct ItemCreateRequest {
    pub(crate) description: String,
    pub(crate) price: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct TableGetRequest {
    pub(crate) id: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct TableCreateRequest {
    pub(crate) table_number: i32,
}
