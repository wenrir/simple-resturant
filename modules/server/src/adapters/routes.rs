//! Routes

use crate::adapters::state::ServerState;
use anyhow::Result;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware,
    middleware::Next,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use log::info;
use serde::Serialize;

#[allow(unused)] // Fallback function is used, false positive.
async fn api_fallback() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "status": "Not Found" })),
    )
}

//fn create_response<T>(result: T) -> Result<Json<T>>
//where
//    T: serde::Serialize,
//{
//    let json = serde_json::to_string_pretty(&result)?;
//    let response = (
//        StatusCode::OK,
//        [
//            (CONTENT_TYPE, "application/json"),
//            (ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
//        ],
//        json,
//    );
//    Ok(response)
//}

async fn is_checked_table_checked_in(req: Request, next: Next) -> Result<Response, StatusCode> {
    //Err(StatusCode::UNAUTHORIZED)
    //if token_is_valid(token) => {
    info!("here..");
    let response = next.run(req).await;
    Ok(response)
    //    }
    //    _ => {
    //        Err(StatusCode::UNAUTHORIZED)
    //    }
}

#[derive(Serialize)]
pub(crate) struct OrderReponse {
    status: String,
}
//async fn order_handler(State(state): State<Arc<ServerState>>) -> Result<Json<OrderReponse>> {
//    state;
//    Ok(Json(OrderReponse {
//        status: "Ok".to_string(),
//    }))
//    //async fn handler() -> Result<()> {
//    //diesel::insert_into(posts::table)
//    //    .values(&new_post)
//    //    .returning(Post::as_returning())
//    //    .get_result(conn)
//    //    .expect("Error saving new post");
//}
async fn order_handler(State(_state): State<ServerState>) -> impl IntoResponse {
    Json(OrderReponse {
        status: "OK".to_string(),
    })
}

fn order_routes() -> Router<ServerState> {
    Router::new()
        .route("/", post({}).get(order_handler))
        .route("/users/:id", get({}))
        .route_layer(middleware::from_fn(is_checked_table_checked_in))

    //Router::new()
    //    .route("/hello", get(order_handler))
    //    .fallback(api_fallback)
}

//TODO CANNOT SHARE PGCONNECTION!!
/// Creates server application routes.
pub(crate) fn routes(state: ServerState) -> Router {
    let router = Router::new().nest("/api/v1/order", order_routes()); // Todo add open api json spec here.
                                                                      //let router = order_routes(router);
    router.with_state(state)
}
