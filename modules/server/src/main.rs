//! main.rs

use application::log::setup_logger;
use fastrace::prelude::{LocalSpan, Span, SpanContext};
use infrastructure::{db::establish_connection, server::Server};
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
        let _connection =
            establish_connection().expect("Unable to establish or migrate database connection!");
        let server = Server::new().await.expect("Unable to setup server!");
        let _ = LocalSpan::enter_with_local_parent("App");

        match server.run().await {
            Ok(_) => {
                info!("Server process finished");
            }
            Err(_) => {
                error!("Server exited unexpectedly");
            }
        }
        // let runtime = tokio::runtime::Builder::new_current_thread()
        //     .enable_all()
        //     .build()
        //     .expect("failed to create Tokio runtime");
        // {
        //     // Spawn the application.
        //     runtime.spawn(async move {
        //         match server.run().await {
        //             Ok(_) => {
        //                 info!("Server process finished");
        //             }
        //             Err(_) => {
        //                 error!("Server exited unexpectedly");
        //             }
        //         }
        //     });
        // }
    }
    fastrace::flush();
}
