use axum::{Router, routing::post};

use crate::{
    app::AppState,
    modules::auth::handler::{login, logout, refresh, register},
};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
}
