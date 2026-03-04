use axum::{Router, routing::get};

use crate::app::AppState;

use super::handler::*;

pub fn todo_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(todos).post(create_todo))
        .route("/{id}", get(get_todo).put(update_todo).delete(delete_todo))
}
