//! adapters/dto/request.rs

use serde::{Deserialize, Serialize};
//use serde::{Deserialize, Serialize};
//#[derive(Debug, Deserialize, Serialize, ToSchema, Dummy)]
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct OrderRequest {
    pub(crate) table_number: i32,
}
