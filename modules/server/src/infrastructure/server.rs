//! infrastructure/server.rs
//! Server module
use anyhow::Result;
use tokio::net::TcpListener;

use crate::adapters::state::State;

/// Creates a server object!
pub(crate) struct Server {
    #[allow(dead_code)] // TODO remove me
    pub(crate) state: State,
    #[allow(dead_code)] // TODO remove me
    socket: TcpListener,
}

impl Server {
    /// Crate a new server.
    pub(crate) async fn new() -> Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:8080").await?; // TODO read me from config.
        Ok(Server {
            // TODO do something useful here ;)
            state: State {},
            socket: listener,
        })
    }

    /// Runs the server.
    #[allow(dead_code)]
    pub(crate) async fn run() {}
}
