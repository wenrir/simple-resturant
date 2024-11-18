//! Routes

use crate::{
    adapters::state::ServerState, application::repo::OrderRepository,
    domain::entities::order::NewOrder,
};
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Json, Router,
};

use super::{
    dto::{request::OrderRequest, response::OrderResponse},
    ServerResult,
};

#[allow(unused)] // Fallback function is used, false positive.
async fn api_fallback() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "status": "Not Found" })),
    )
}

async fn is_checked_table_checked_in(req: Request, next: Next) -> ServerResult<Response> {
    let response = next.run(req).await;
    Ok(response)
}
async fn get_order(
    State(_state): State<ServerState>,
    Json(_req): Json<OrderRequest>,
) -> ServerResult<Json<OrderResponse>> {
    let result = OrderResponse {
        status: "OK".to_string(),
    };
    // TODO call repo her
    Ok(Json(result))
}

async fn create_order(
    State(state): State<ServerState>,
    Json(req): Json<OrderRequest>,
) -> ServerResult<Json<OrderResponse>> {
    let order = NewOrder {
        table_number: &req.table_number,
    };
    // create order ..
    match state.order_repository.create_order(&order) {
        Ok(res) => Ok(Json(OrderResponse {
            status: res.table_number.to_string(),
        })),
        Err(err) => Err(err),
    }
}
fn order_routes() -> Router<ServerState> {
    Router::new()
        .route("/:id", post(create_order).get(|| async {}))
        .route(
            "/:id/:items",
            get(get_order)
                .patch(|| async {
                    // Update order
                })
                .delete(|| async {
                    // delete order
                }),
        )
        .route_layer(middleware::from_fn(is_checked_table_checked_in))
}

/// Creates server application routes.
pub(crate) fn routes(state: ServerState) -> Router {
    let router = Router::new().nest("/api/v1/orders", order_routes()); // Todo add open api json spec here.
    router.fallback(api_fallback).with_state(state)
}
