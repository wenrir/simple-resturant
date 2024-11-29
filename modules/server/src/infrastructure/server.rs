//! infrastructure/server.rs
//! Server module
use anyhow::Result;
use std::env::var;
use tokio::net::TcpListener;

use crate::adapters::routes::routes;
use crate::adapters::state::ServerState;
use crate::application::config::{HOST_PORT, HOST_URL};

use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;

/// Creates a server object!
pub(crate) struct Server {
    pub(crate) state: ServerState,
    socket: TcpListener,
}

impl Server {
    /// Crate a new server.
    pub(crate) async fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Result<Server> {
        //let connection = connect()?;
        let host = var("HOST_URL").unwrap_or_else(|_| HOST_URL.to_string());
        let port = var("HOST_PORT").unwrap_or_else(|_| HOST_PORT.to_string());
        let listener = TcpListener::bind(format!("{}:{}", host, port)).await?; // TODO read me from config.
        Ok(Server {
            // TODO do something useful here ;)
            state: ServerState::new(pool)?,
            socket: listener,
        })
    }

    /// Runs the server.
    pub(crate) async fn run(self) -> Result<()> {
        let router = routes(self.state);
        axum::serve(self.socket, router).await?;
        Ok(())
    }
}
