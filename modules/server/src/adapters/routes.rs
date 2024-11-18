//! Routes

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
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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

/// Find order by table number.
#[fastrace::trace]
#[utoipa::path(
        get,
        request_body = i32,
        path = "/api/v1/orders/:id",
        responses(
            (status = 200, description = "Success found order", body = [OrderResponse]),
            (status = 500, description = "Internal server error", body = [crate::adapters::ServerError])
        )
    )]
async fn get_orders(
    State(state): State<ServerState>,
    Path(id): Path<i32>,
) -> ServerResult<Json<OrderResponse>> {
    match state.order_repository.find_by_table_number(id) {
        Ok(res) => Ok(Json(OrderResponse { order: res })),
        Err(err) => Err(err),
    }
}

/// Find order item by table number.
#[fastrace::trace]
#[utoipa::path(
        get,
        request_body = (i32, i32),
        path = "/api/v1/orders/:id/:item",
        responses(
            (status = 200, description = "Success found item in order", body = [OrderResponse]),
            (status = 500, description = "Internal server error", body = [crate::adapters::ServerError])
        )
    )]
async fn get_order_by_item(
    State(state): State<ServerState>,
    Path((table_id, item_id)): Path<(i32, i32)>,
) -> ServerResult<Json<OrderResponse>> {
    match state.order_repository.find_specific_item(table_id, item_id) {
        Ok(res) => Ok(Json(OrderResponse { order: res })),
        Err(err) => Err(err),
    }
}

/// Create an order.
#[utoipa::path(
        post,
        request_body = OrderCreateRequest,
        path = "/api/v1/orders/",
        responses(
            (status = 200, description = "Success created order", body = [String]),
            (status = 500, description = "Internal server error", body = [crate::adapters::ServerError])
        )
    )]
async fn create_order(
    State(state): State<ServerState>,
    Json(req): Json<OrderCreateRequest>,
) -> ServerResult<String> {
    use chrono::prelude::*;
    let order = NewOrder {
        table_number: &req.table_number,
        item_id: &req.item_id,
        customer_id: &req.customer_id,
        published_at: &Local::now().to_rfc3339(),
        quantity: &req.quantity,
    };
    match state.order_repository.create(&order) {
        Ok(_) => Ok("OK".to_string()),
        Err(err) => Err(err),
    }
}

/// Delete an order.
#[utoipa::path(
        delete,
        request_body = (i32,i32),
        path = "/api/v1/orders/:id",
        responses(
            (status = 200, description = "Success deleted order", body = [String]),
            (status = 500, description = "Internal server error", body = [crate::adapters::ServerError])
        )
    )]
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
/// Get item.
#[fastrace::trace]
#[utoipa::path(
        get,
        request_body = ItemGetRequest,
        path = "/api/v1/item",
        responses(
            (status = 200, description = "Successfully found item", body = [ItemResponse]),
            (status = 500, description = "Internal server error", body = [crate::adapters::ServerError])
        )
    )]
async fn get_item(
    State(state): State<ServerState>,
    Json(req): Json<ItemGetRequest>,
) -> ServerResult<Json<ItemResponse>> {
    match state.item_repository.get(&req.id) {
        Ok(res) => Ok(Json(ItemResponse { item: res })),
        Err(err) => Err(err),
    }
}

/// Create item.
#[fastrace::trace]
#[utoipa::path(
        post,
        request_body = ItemCreateRequest,
        path = "/api/v1/item",
        responses(
            (status = 200, description = "Successfully created item", body = [ItemResponse]),
            (status = 500, description = "Internal server error", body = [crate::adapters::ServerError])
        )
    )]
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

/// Get customer.
#[utoipa::path(
        get,
        request_body = CustomerGetRequest,
        path = "/api/v1/customer",
        responses(
            (status = 200, description = "Successfully found item", body = [CustomerResponse]),
            (status = 500, description = "Internal server error", body = [crate::adapters::ServerError])
        )
    )]
async fn get_customer(
    State(state): State<ServerState>,
    Json(req): Json<CustomerGetRequest>,
) -> ServerResult<Json<CustomerResponse>> {
    match state.customer_repository.get(&req.id) {
        Ok(res) => Ok(Json(CustomerResponse { customer: res })),
        Err(err) => Err(err),
    }
}

/// Create customer.
#[utoipa::path(
        post,
        path = "/api/v1/customer/check_in",
        responses(
            (status = 200, description = "Checks in a customer", body = [CustomerResponse]),
            (status = 500, description = "Internal server error", body = [crate::adapters::ServerError])
        )
    )]
async fn create_customer(State(state): State<ServerState>) -> ServerResult<Json<CustomerResponse>> {
    use chrono::prelude::*;
    let customer = &NewCustomer {
        checked_in_time: &Local::now().to_rfc3339(),
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
#[derive(OpenApi)]
#[openapi(
    info(
        version = "v0.1.0",
        title = "Simple Resturant API",
    ),
    paths(

        // Customer endpoints
        get_customer,
        create_customer,
        // Item endpoints
        get_item,
        create_item,

        // Order endpoints
        create_order,
        get_orders,
        delete_order,
        get_order_by_item,
    ),
    components(
        schemas(
        CustomerGetRequest, ItemCreateRequest, ItemGetRequest, OrderCreateRequest,
        CustomerResponse, ItemResponse, OrderResponse,
        )
    ),
)]
pub(crate) struct Doc {}

/// Creates server application routes.
pub(crate) fn routes(state: ServerState) -> Router {
    let router = Router::new()
        .nest("/api/v1/orders", order_routes())
        .nest("/api/v1/items", item_routes())
        .nest("/api/v1/tables", customer_routes())
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", Doc::openapi()));
    router.fallback(api_fallback).with_state(state)
}
