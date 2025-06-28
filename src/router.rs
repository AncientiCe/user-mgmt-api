use axum::{Router, middleware, routing::{get, patch, post}};
use tower_http::cors::CorsLayer;
use crate::AppState;
use crate::handlers::*;
use crate::middleware::auth_middleware;

pub fn create_router(state: AppState) -> Router {
    let protected_routes = Router::new()
        .route("/me", get(get_me_handler))
        .route("/users/:id", get(get_user_handler))
        .route("/users/:id", patch(update_user_handler))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .merge(protected_routes)
        .with_state(state)
        .layer(CorsLayer::permissive())
}
