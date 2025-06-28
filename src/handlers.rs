use axum::{extract::{Path, State}, response::Json};
use crate::models::*;
use crate::db::*;
use crate::jwt::*;
use crate::error::*;
use crate::config::Config;
use crate::AppState;
use uuid::Uuid;
use bcrypt;
use axum::Extension;

pub async fn register_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<User>> {
    let user = create_user(&state.db, req).await?;
    Ok(Json(user))
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    let (user, password_hash) = get_user_by_email(&state.db, &req.email).await?;

    if !bcrypt::verify(&req.password, &password_hash)? {
        return Err(UserError::InvalidCredentials);
    }

    let (token, expires_at) = create_jwt(&user, &state.config)?;

    Ok(Json(LoginResponse {
        token,
        user,
        expires_at,
    }))
}

pub async fn get_me_handler(
    Extension(user): Extension<User>,
) -> Result<Json<User>> {
    Ok(Json(user))
}

pub async fn get_user_handler(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<User>> {
    let user = get_user_by_id(&state.db, user_id).await?;
    Ok(Json(user))
}

pub async fn update_user_handler(
    Extension(current_user): Extension<User>,
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<User>> {
    if current_user.id != user_id {
        return Err(UserError::InvalidCredentials);
    }

    let updated_user = update_user(&state.db, user_id, req).await?;
    Ok(Json(updated_user))
}
