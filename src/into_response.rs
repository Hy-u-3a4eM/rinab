use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug)]
pub enum Error {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    Database(sqlx::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::WrongCredentials => (StatusCode::UNAUTHORIZED, String::from("Wrong credentials")),
            Error::MissingCredentials => (StatusCode::BAD_REQUEST, String::from("Missing credentials")),
            Error::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, String::from("Token creation error")),
            Error::InvalidToken => (StatusCode::BAD_REQUEST, String::from("Invalid token")),
            Error::Database(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", err))
        };
        let body = Json(json!({
            "status": "error",
            "error": {
                "code": status,
                "message": error_message,
            },
        }));
        (status, body).into_response()
    }
}
