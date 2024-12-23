//! main.rs

use std::process::exit;

use application::log::setup_logger;
use fastrace::prelude::{LocalSpan, Span, SpanContext};
use infrastructure::{
    db::{get_connection_pool, migrate},
    server::Server,
};
use log::{error, info};
mod adapters;
mod application;
mod domain;
mod infrastructure;

/// Main entrypoint of the server (bin).
#[tokio::main]
async fn main() {
    setup_logger();
    {
        let parent = SpanContext::random();
        let root = Span::root("server", parent);
        let _ = root.set_local_parent();
        let _ = LocalSpan::enter_with_local_parent("Setup");
        match migrate() {
            Ok(_) => info!("Successfully migrated db!"),
            Err(e) => {
                error!("Failed to migrate db {}", e);
                exit(1)
            }
        }
        let pool = get_connection_pool();
        let server = Server::new(pool).await.expect("Unable to setup server!");
        let _ = LocalSpan::enter_with_local_parent("App");

        match server.run().await {
            Ok(_) => {
                info!("Server process finished");
            }
            Err(_) => {
                error!("Server exited unexpectedly");
            }
        }
    }
    fastrace::flush();
}
