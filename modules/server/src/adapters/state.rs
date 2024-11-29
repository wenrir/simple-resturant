//! adapter/state.rs

use anyhow::Result;
use serde::Deserialize;

use super::factories::{ItemFactory, OrderFactory, TableFactory};

/// Server state.
#[derive(Clone, Deserialize)]
pub(crate) struct ServerState {
    pub(crate) order_repository: OrderFactory,
    pub(crate) item_repository: ItemFactory,
    pub(crate) table_repository: TableFactory,
}

impl ServerState {
    pub(crate) fn new() -> Result<Self> {
        Ok(ServerState {
            order_repository: OrderFactory {},
            item_repository: ItemFactory {},
            table_repository: TableFactory {},
        })
    }
}
