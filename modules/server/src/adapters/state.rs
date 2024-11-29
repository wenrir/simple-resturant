//! adapter/state.rs

use anyhow::Result;

use super::factories::{ItemFactory, OrderFactory, TableFactory};
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;

/// Server state.
#[derive(Clone, Debug)]
pub(crate) struct ServerState {
    pub(crate) order_repository: OrderFactory,
    pub(crate) item_repository: ItemFactory,
    pub(crate) table_repository: TableFactory,
}

impl ServerState {
    pub(crate) fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Result<Self> {
        // TODO: Introduce lifetimes instead of cloning the connection!
        Ok(ServerState {
            order_repository: OrderFactory {
                connection_pool: pool.clone(),
            },
            item_repository: ItemFactory {
                connection_pool: pool.clone(),
            },
            table_repository: TableFactory {
                connection_pool: pool.clone(),
            },
        })
    }
}
