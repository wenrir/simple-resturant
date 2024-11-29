//! Routes

use crate::{
    adapters::state::ServerState,
    application::repo::{ItemRepository, OrderRepository, TableRepository},
    domain::entities::{
        item::NewItem,
        order::{NewOrder, Order},
        table::NewTable,
    },
};
use axum::{
    extract::{Path, Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::{delete, get, post},
    Json, Router,
};
use rand::Rng;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use super::{
    dto::{
        request::{ItemCreateRequest, OrderCreateRequest, TableCreateRequest, TableGetRequest},
        response::{ItemResponse, ItemsResponse, OrderResponse, TableResponse, TablesResponse},
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
async fn get_order_by_id(
    State(state): State<ServerState>,
    Path(id): Path<i32>,
) -> ServerResult<Json<OrderResponse>> {
    match state.order_repository.find(&id) {
        Ok(res) => Ok(Json(OrderResponse { data: res })),
        Err(err) => Err(err),
    }
}

/// Find all orders
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
async fn get_orders(State(state): State<ServerState>) -> ServerResult<Json<OrderResponse>> {
    match state.order_repository.all() {
        Ok(res) => Ok(Json(OrderResponse { data: res })),
        Err(err) => Err(err),
    }
}

/// Create an order.
#[utoipa::path(
        post,
        request_body = Vec<OrderCreateRequest>,
        path = "/api/v1/orders",
        responses(
            (status = 200, description = "Success created order", body = [String]),
            (status = 500, description = "Internal server error", body = [crate::adapters::ServerError])
        )
    )]
async fn create_order(
    State(state): State<ServerState>,
    Json(reqs): Json<Vec<OrderCreateRequest>>,
) -> ServerResult<String> {
    use chrono::prelude::*;
    let mut responses = Vec::new();
    for req in reqs.iter() {
        let order = NewOrder {
            item_id: &req.item_id,
            table_id: &req.table_id,
            published_at: &Local::now().to_rfc3339(),
            quantity: &req.quantity,
        };
        match state.order_repository.create(&order) {
            Ok(_) => responses.push(format!(
                "Order for item_id {} added successfully.",
                req.item_id
            )),
            Err(err) => responses.push(format!(
                "Failed to add order for item_id {}: {:?}",
                req.item_id, err
            )),
        }
    }
    Ok(responses.join("\n"))
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
        .route("/", post(create_order).get(get_orders))
        .route("/:id", get(get_order_by_id).delete(delete_order))
        .route_layer(middleware::from_fn(is_checked_table_checked_in))
}

// TODO These can be converted to macros
/// Get items.
#[fastrace::trace]
#[utoipa::path(
        get,
        path = "/api/v1/items",
        responses(
            (status = 200, description = "Successfully found items", body = [ItemsResponse]),
            (status = 500, description = "Internal server error", body = [crate::adapters::ServerError])
        )
    )]
async fn get_items(State(state): State<ServerState>) -> ServerResult<Json<ItemsResponse>> {
    match state.item_repository.all() {
        Ok(res) => Ok(Json(ItemsResponse { data: res })),
        Err(err) => Err(err),
    }
}

/// Get specific item.
#[fastrace::trace]
#[utoipa::path(
        get,
        path = "/api/v1/items/:id",
        responses(
            (status = 200, description = "Successfully found item", body = [ItemResponse]),
            (status = 500, description = "Internal server error", body = [crate::adapters::ServerError])
        )
    )]
async fn get_item(
    State(state): State<ServerState>,
    Path(id): Path<i32>,
) -> ServerResult<Json<ItemResponse>> {
    match state.item_repository.get(&id) {
        Ok(res) => Ok(Json(ItemResponse { data: res })),
        Err(err) => Err(err),
    }
}

/// Create item.
#[fastrace::trace]
#[utoipa::path(
        post,
        request_body = ItemCreateRequest,
        path = "/api/v1/items",
        responses(
            (status = 200, description = "Successfully created item", body = [ItemResponse]),
            (status = 500, description = "Internal server error", body = [crate::adapters::ServerError])
        )
    )]
async fn create_item(
    State(state): State<ServerState>,
    Json(req): Json<ItemCreateRequest>,
) -> ServerResult<Json<ItemResponse>> {
    let mut rng = rand::thread_rng();
    let item = NewItem {
        description: &req.description,
        estimated_minutes: &rng.gen_range(5..=15),
    };
    match state.item_repository.create(&item) {
        Ok(res) => Ok(Json(ItemResponse { data: res })),
        Err(err) => Err(err),
    }
}

fn item_routes() -> Router<ServerState> {
    Router::new()
        .route("/", post(create_item).get(get_items))
        .route("/:id", get(get_item))
}

/// Get table.
#[utoipa::path(
        get,
        path = "/api/v1/tables/:id",
        responses(
            (status = 200, description = "Successfully found item", body = [TableResponse]),
            (status = 500, description = "Internal server error", body = [crate::adapters::ServerError])
        )
    )]
async fn get_table(
    State(state): State<ServerState>,
    Path(id): Path<i32>,
) -> ServerResult<Json<TableResponse>> {
    match state.table_repository.get(&id) {
        Ok(res) => Ok(Json(TableResponse { data: res })),
        Err(err) => Err(err),
    }
}

/// Get table orders.
#[utoipa::path(
        get,
        path = "/api/v1/tables/:id/orders",
        responses(
            (status = 200, description = "Successfully found item", body = [OrderResponse]),
            (status = 500, description = "Internal server error", body = [crate::adapters::ServerError])
        )
    )]
async fn get_table_orders(
    State(state): State<ServerState>,
    Path(id): Path<i32>,
) -> ServerResult<Json<OrderResponse>> {
    match state.order_repository.find_table(&id) {
        Ok(res) => Ok(Json(OrderResponse { data: res })),
        Err(err) => Err(err),
    }
}

/// Get table order.
#[utoipa::path(
        get,
        path = "/api/v1/tables/:id/orders/:id",
        responses(
            (status = 200, description = "Successfully found order", body = [OrderResponse]),
            (status = 500, description = "Internal server error", body = [crate::adapters::ServerError])
        )
    )]
async fn get_table_order(
    State(state): State<ServerState>,
    Path(ids): Path<(i32, i32)>,
) -> ServerResult<Json<OrderResponse>> {
    match state.order_repository.find_table(&ids.0) {
        Ok(res) => {
            let r: Vec<Order> = res.into_iter().filter(|i| i.id == ids.1).collect();
            Ok(Json(OrderResponse { data: r }))
        }
        Err(err) => Err(err),
    }
}

/// Get table items.
#[utoipa::path(
        get,
        path = "/api/v1/tables/:id/items/:id",
        responses(
            (status = 200, description = "Successfully found order", body = [ItemsResponse]),
            (status = 500, description = "Internal server error", body = [crate::adapters::ServerError])
        )
    )]
async fn get_table_items(
    State(state): State<ServerState>,
    Path(ids): Path<(i32, i32)>,
) -> ServerResult<Json<ItemsResponse>> {
    match state.order_repository.find_table(&ids.0) {
        Ok(res) => {
            let r: Vec<Order> = res.into_iter().filter(|i| i.item_id == ids.1).collect();
            let mut items = vec![];
            for order in r {
                match state.item_repository.get(&order.item_id) {
                    Ok(item) => items.push(item),
                    Err(err) => {
                        eprintln!("Failed to fetch item with ID {}: {:?}", order.item_id, err);
                        return Err(err);
                    }
                }
            }

            Ok(Json(ItemsResponse { data: items }))
        }
        Err(err) => Err(err),
    }
}

/// Delete table order.
#[utoipa::path(
        delete,
        path = "/api/v1/tables/:id/orders/:id",
        responses(
            (status = 200, description = "Successfully deleted item", body = [String]),
            (status = 500, description = "Internal server error", body = [crate::adapters::ServerError])
        )
    )]
async fn delete_table_order(
    State(state): State<ServerState>,
    Path(ids): Path<(i32, i32)>,
) -> ServerResult<String> {
    match state.order_repository.delete_table_order(&ids.0, &ids.1) {
        Ok(res) => Ok(res),
        Err(err) => Err(err),
    }
}

/// Get tables.
#[utoipa::path(
        get,
        path = "/api/v1/tables",
        responses(
            (status = 200, description = "Successfully found item", body = [TablesResponse]),
            (status = 500, description = "Internal server error", body = [crate::adapters::ServerError])
        )
    )]
async fn get_tables(State(state): State<ServerState>) -> ServerResult<Json<TablesResponse>> {
    match state.table_repository.all() {
        Ok(res) => Ok(Json(TablesResponse { data: res })),
        Err(err) => Err(err),
    }
}

/// Checks in a table to table.
#[utoipa::path(
        post,
        request_body = TableCreateRequest,
        path = "/api/v1/tables/check_in",
        responses(
            (status = 200, description = "Checks in a table", body = [TableResponse]),
            (status = 500, description = "Internal server error", body = [crate::adapters::ServerError])
        )
    )]
async fn create_table(
    State(state): State<ServerState>,
    Json(req): Json<TableCreateRequest>,
) -> ServerResult<Json<TableResponse>> {
    use chrono::prelude::*;
    let table = &NewTable {
        table_number: &req.table_number,
        checked_in_time: &Local::now().to_rfc3339(),
        total: &0_i32,
    };
    match state.table_repository.create(table) {
        Ok(res) => Ok(Json(TableResponse { data: res })),
        Err(err) => Err(err),
    }
}

fn table_routes() -> Router<ServerState> {
    Router::new()
        .route("/", get(get_tables))
        .route("/:id", get(get_table))
        .route("/:id/orders", get(get_table_orders))
        .route(
            "/:id/orders/:id",
            delete(delete_table_order).get(get_table_order),
        )
        .route("/:id/items/:id", get(get_table_items))
        .route("/check_in", post(create_table))
}
#[derive(OpenApi)]
#[openapi(
    info(
        version = "v0.1.0",
        title = "Simple Resturant API",
    ),
    paths(

        // Table endpoints
        get_table,
        create_table,
        // Item endpoints
        get_item,
        get_items,
        create_item,

        // Order endpoints
        create_order,
        get_order_by_id,
        delete_order,
    ),
    components(
        schemas(
        TableGetRequest, ItemCreateRequest, OrderCreateRequest,
        TableResponse, ItemResponse, OrderResponse,
        )
    ),
)]
pub(crate) struct Doc {}

/// Creates server application routes.
pub(crate) fn routes(state: ServerState) -> Router {
    let router = Router::new()
        .nest("/api/v1/orders", order_routes())
        .nest("/api/v1/items", item_routes())
        .nest("/api/v1/tables", table_routes())
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", Doc::openapi()));
    router.fallback(api_fallback).with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;
    fn get_test_routes() -> Router {
        routes(ServerState::new().expect("unable to create server state."))
    }
    use serde_json::json;
    fn build_test_server() -> TestServer {
        let r = get_test_routes();

        TestServer::builder()
            .save_cookies()
            .expect_success_by_default()
            .mock_transport()
            .build(r)
            .unwrap()
    }

    #[tokio::test]
    async fn test_create_item() {
        let server = build_test_server();
        {
            let response = server
                .post("/api/v1/items")
                .json(&json!({
                    "description": "Some good tasting item!",
                }))
                .await;
            assert_eq!(response.status_code(), StatusCode::OK);
        }
        {
            let response = server.get("/api/v1/items/1").await;
            assert_eq!(response.status_code(), StatusCode::OK);
        }
        {
            let response = server.get("/api/v1/items/10").expect_failure().await;
            assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    #[tokio::test]
    async fn test_create_table() {
        let server = build_test_server();
        {
            let response = server
                .post("/api/v1/tables/check_in")
                .json(&json!({"table_number": 1}))
                .await;
            assert_eq!(response.status_code(), StatusCode::OK);
        }
        {
            let response = server
                .get("/api/v1/tables")
                .json(&json!({
                    "id": 1,
                }))
                .await;
            assert_eq!(response.status_code(), StatusCode::OK);
        }
    }

    #[tokio::test]
    async fn test_create_order() {
        let server = build_test_server();
        {
            let response = server
                .post("/api/v1/tables/check_in")
                .json(&json!({

                        "table_number": 10,
                }))
                .await;
            assert_eq!(response.status_code(), StatusCode::OK);
        }
        {
            let response = server
                .post("/api/v1/items")
                .json(&json!({
                    "description": "Some good tasting item!",
                }))
                .await;
            assert_eq!(response.status_code(), StatusCode::OK);
        }

        {
            let response = server
                .post("/api/v1/orders")
                .json(&json!([
                    {"item_id": 1,
                    "table_id": 2,
                    "quantity": 10,},
                    {"item_id": 1,
                    "table_id": 2,
                    "quantity": 1,},
                    {"item_id": 1,
                    "table_id": 2,
                    "quantity": 5,},
                    {"item_id": 1,
                    "table_id": 2,
                    "quantity": 6,},
                    {"item_id": 1,
                    "table_id": 2,
                    "quantity": 100,},
                    {"item_id": 1,
                    "table_id": 2,
                    "quantity": 50,},
                    {"item_id": 1,
                    "table_id": 2,
                    "quantity": 70,},
                    {"item_id": 1,
                    "table_id": 2,
                    "quantity": 1,},
                ]))
                .await;
            assert_eq!(response.status_code(), StatusCode::OK);
        }
    }
}
