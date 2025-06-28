pub mod config;
pub mod error;
pub mod jwt;
pub mod models;
pub mod db;
pub mod handlers;
pub mod middleware;
pub mod router;
use sqlx::PgPool;
use crate::config::Config;
use crate::db::{setup_database, get_user_by_id};
use crate::models::{User};
use crate::error::{Result};
use crate::router::create_router;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Config,
}

pub async fn create_user_manager(db: PgPool, config: Option<Config>) -> Result<axum::Router> {
    setup_database(&db).await?;
    let config = config.unwrap_or_default();
    let state = AppState { db, config };
    Ok(create_router(state))
}

pub async fn get_user_from_token(db: &PgPool, token: &str, config: &Config) -> Result<User> {
    let claims = crate::jwt::verify_jwt(token, config)?;
    let user_id = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|_| crate::error::UserError::InvalidToken)?;
    get_user_by_id(db, user_id).await
}
