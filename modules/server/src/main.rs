//! main.rs
use application::log::setup_logger;
use fastrace::prelude::{LocalSpan, Span, SpanContext};
use infrastructure::{db::establish_connection, server::Server};
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
        let _server = Server::new();
        let _ = LocalSpan::enter_with_local_parent("App");
    }
    fastrace::flush();
}
