pub(crate) mod dto;
pub(crate) mod factories;
pub(crate) mod routes;
pub(crate) mod state;

use anyhow::Result;
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use diesel::{Connection, PgConnection};
use serde::Serialize;
use std::env::var;

pub(crate) fn db_connect() -> Result<PgConnection> {
    let database_url = var("DATABASE_URL").expect("Database URL needs to be set!");

    let conn = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    Ok(conn)
}

#[derive(Serialize)]
pub(crate) struct ServerError {
    error: String,
}
pub(crate) type ServerResult<T = ()> = Result<T, ServerError>;

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let code = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
        (code, Json(self)).into_response()
    }
}
