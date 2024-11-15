//! Routes

use crate::adapters::state::State;
use axum::{http::StatusCode, routing::get, Json, Router};

#[allow(unused)] // Fallback function is used, false positive.
async fn api_fallback() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "status": "Not Found" })),
    )
}

fn order_routes(router: Router<State>) -> Router<State> {
    router
        .route("/api/v1/hello", get(|| async { "Hello, World!" }))
        .fallback(api_fallback)
}

/// Creates server application routes.
pub(crate) fn routes(state: State) -> Router {
    //router.route("/api/v1/order", post({}))
    let router = Router::new(); // Todo add open api json spec here.
    let router = order_routes(router);
    router.with_state(state)
}
