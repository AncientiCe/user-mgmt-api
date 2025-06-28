use axum::{extract::State, http::header};
use crate::AppState;
use crate::error::*;
use crate::jwt::*;
use crate::db::get_user_by_id;
use uuid::Uuid;
use axum::middleware::Next;
use axum::http::{Request};
use axum::response::Response;

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> std::result::Result<Response, UserError> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let token = auth_header
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .ok_or(UserError::InvalidToken)?;

    let claims = verify_jwt(token, &state.config)?;
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| UserError::InvalidToken)?;

    let user = get_user_by_id(&state.db, user_id).await?;
    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
