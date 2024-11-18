//! Routes

use std::time::SystemTime;

use crate::{
    adapters::state::ServerState,
    application::repo::{CustomerRepository, ItemRepository, OrderRepository},
    domain::entities::{customer::NewCustomer, item::NewItem, order::NewOrder},
};
use axum::{
    extract::{Path, Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Json, Router,
};

use super::{
    dto::{
        request::{CustomerGetRequest, ItemCreateRequest, ItemGetRequest, OrderCreateRequest},
        response::{CustomerResponse, ItemResponse, OrderResponse},
    },
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

#[fastrace::trace]
async fn get_orders(
    State(state): State<ServerState>,
    Path(id): Path<i32>,
) -> ServerResult<Json<OrderResponse>> {
    match state.order_repository.find_by_table_number(id) {
        Ok(res) => Ok(Json(OrderResponse { order: res })),
        Err(err) => Err(err),
    }
}

#[fastrace::trace]
async fn get_order_by_item(
    State(state): State<ServerState>,
    Path((table_id, item_id)): Path<(i32, i32)>,
) -> ServerResult<Json<OrderResponse>> {
    match state.order_repository.find_specific_item(table_id, item_id) {
        Ok(res) => Ok(Json(OrderResponse { order: res })),
        Err(err) => Err(err),
    }
}

async fn create_order(
    State(state): State<ServerState>,
    Json(req): Json<OrderCreateRequest>,
) -> ServerResult<String> {
    let order = NewOrder {
        table_number: &req.table_number,
        item_id: &req.item_id,
        customer_id: &req.customer_id,
        published_at: &SystemTime::now(),
        quantity: &req.quantity,
    };
    match state.order_repository.create(&order) {
        Ok(_) => Ok("OK".to_string()),
        Err(err) => Err(err),
    }
}

async fn delete_order(
    State(state): State<ServerState>,
    Path(id): Path<i32>,
) -> ServerResult<String> {
    match state.order_repository.delete(&id) {
        Ok(_) => Ok("OK".to_string()),
        Err(err) => Err(err),
    }
}
fn order_routes() -> Router<ServerState> {
    Router::new()
        .route("/", post(create_order))
        .route("/:id", get(get_orders).delete(delete_order))
        .route("/:id/:item", get(get_order_by_item))
        .route_layer(middleware::from_fn(is_checked_table_checked_in))
}

// TODO These can be converted to macros
#[fastrace::trace]
async fn get_item(
    State(state): State<ServerState>,
    Json(req): Json<ItemGetRequest>,
) -> ServerResult<Json<ItemResponse>> {
    match state.item_repository.get(&req.id) {
        Ok(res) => Ok(Json(ItemResponse { item: res })),
        Err(err) => Err(err),
    }
}

#[fastrace::trace]
async fn create_item(
    State(state): State<ServerState>,
    Json(req): Json<ItemCreateRequest>,
) -> ServerResult<Json<ItemResponse>> {
    let item = NewItem {
        description: &req.description,
        estimated_minutes: &req.estimated_minutes,
    };
    match state.item_repository.create(&item) {
        Ok(res) => Ok(Json(ItemResponse { item: res })),
        Err(err) => Err(err),
    }
}

fn item_routes() -> Router<ServerState> {
    Router::new().route("/", post(create_item).get(get_item))
}

async fn get_customer(
    State(state): State<ServerState>,
    Json(req): Json<CustomerGetRequest>,
) -> ServerResult<Json<CustomerResponse>> {
    match state.customer_repository.get(&req.id) {
        Ok(res) => Ok(Json(CustomerResponse { customer: res })),
        Err(err) => Err(err),
    }
}

async fn create_customer(State(state): State<ServerState>) -> ServerResult<Json<CustomerResponse>> {
    let customer = &NewCustomer {
        checked_in_time: &SystemTime::now(),
        total: &0_i32,
    };
    match state.customer_repository.create(customer) {
        Ok(res) => Ok(Json(CustomerResponse { customer: res })),
        Err(err) => Err(err),
    }
}

fn customer_routes() -> Router<ServerState> {
    Router::new()
        .route("/", get(get_customer))
        .route("/check_in", post(create_customer))
}

/// Creates server application routes.
pub(crate) fn routes(state: ServerState) -> Router {
    let router = Router::new()
        .nest("/api/v1/orders", order_routes())
        .nest("/api/v1/items", item_routes())
        .nest("/api/v1/tables", customer_routes()); // Todo add open api json spec here.
    router.fallback(api_fallback).with_state(state)
}
