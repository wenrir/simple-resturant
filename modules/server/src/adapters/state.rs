//! adapter/state.rs

use anyhow::Result;
use serde::Deserialize;

use super::factories::{CustomerFactory, ItemFactory, OrderFactory};

/// Server state.
#[derive(Clone, Deserialize)]
pub(crate) struct ServerState {
    pub(crate) order_repository: OrderFactory,
    pub(crate) item_repository: ItemFactory,
    pub(crate) customer_repository: CustomerFactory,
}

impl ServerState {
    pub(crate) fn new() -> Result<Self> {
        Ok(ServerState {
            order_repository: OrderFactory {},
            item_repository: ItemFactory {},
            customer_repository: CustomerFactory {},
        })
    }
}
