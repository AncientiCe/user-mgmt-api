use axum::http::StatusCode;
use axum::response::{IntoResponse, Response, Json};
use serde_json::json;

#[derive(thiserror::Error, Debug)]
pub enum UserError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("Bcrypt error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token expired")]
    TokenExpired,
    #[error("Email already exists")]
    EmailExists,
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Internal server error")]
    Internal,
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            UserError::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            UserError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            UserError::InvalidToken | UserError::TokenExpired => {
                (StatusCode::UNAUTHORIZED, "Invalid or expired token")
            }
            UserError::EmailExists => (StatusCode::CONFLICT, "Email already exists"),
            UserError::Validation(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            UserError::Database(_) | UserError::Jwt(_) | UserError::Bcrypt(_) | UserError::Internal => {
                tracing::error!("Internal error: {}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, UserError>;
