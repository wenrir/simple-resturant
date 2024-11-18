//! adapters/dto/responses.rs
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct OrderResponse {
    pub(crate) status: String,
}
