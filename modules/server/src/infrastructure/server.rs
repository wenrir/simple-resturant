//! infrastructure/server.rs
//! Server module
use anyhow::Result;
use std::env::var;
use tokio::net::TcpListener;

use crate::adapters::routes::routes;
use crate::adapters::state::State;
use crate::application::config::{HOST_PORT, HOST_URL};

/// Creates a server object!
pub(crate) struct Server {
    #[allow(unused)]
    pub(crate) state: State,
    #[allow(unused)]
    socket: TcpListener,
}

impl Server {
    /// Crate a new server.
    pub(crate) async fn new() -> Result<Server> {
        let host = var("HOST_URL").unwrap_or_else(|_| HOST_URL.to_string());
        let port = var("HOST_PORT").unwrap_or_else(|_| HOST_PORT.to_string());
        let listener = TcpListener::bind(format!("{}:{}", host, port)).await?; // TODO read me from config.
        Ok(Server {
            // TODO do something useful here ;)
            state: State {},
            socket: listener,
        })
    }

    /// Runs the server.
    pub(crate) async fn run(self) -> Result<()> {
        axum::serve(self.socket, routes(self.state)).await?;
        Ok(())
    }
}
