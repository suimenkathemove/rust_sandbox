use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub enum AuthError {
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        // TODO
        (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response()
    }
}
