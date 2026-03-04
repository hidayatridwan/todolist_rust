use axum::{Router, middleware, routing::get};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

use crate::{
    config::jwt::JwtConfig,
    middleware::request_id::request_id_middleware,
    modules::{auth::routes::auth_routes, todo::routes::todo_routes},
};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt: JwtConfig,
}

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .nest("/auth", auth_routes())
        .nest("/todos", todo_routes())
        .layer(middleware::from_fn(request_id_middleware))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

async fn health_check() -> &'static str {
    "OK"
}
