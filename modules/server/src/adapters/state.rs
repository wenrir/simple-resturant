//! adapter/state.rs

use anyhow::Result;

/// Server state.
#[derive(Clone)]
pub(crate) struct ServerState {}

impl ServerState {
    pub(crate) async fn new() -> Result<Self> {
        Ok(ServerState {})
    }
}
