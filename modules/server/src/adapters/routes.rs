//! Routes

use crate::domain::server::State;
use axum::{routing::post, Router};

/// Creates server application routes.
#[allow(unused)]
pub(crate) fn routes(router: Router<State>) -> axum::Router<State> {
    router.route("/api/v1/order", post({}))
}
