//! adapter/state.rs

use anyhow::Result;
use serde::Deserialize;

use super::factories::OrderFactory;

/// Server state.
#[derive(Clone, Deserialize)]
#[allow(dead_code)]
pub(crate) struct ServerState {
    pub(crate) order_repository: OrderFactory,
}

impl ServerState {
    pub(crate) async fn new() -> Result<Self> {
        Ok(ServerState {
            order_repository: OrderFactory {},
        })
    }
}
