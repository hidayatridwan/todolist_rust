use axum::{
    Json,
    extract::{Path, Query, State},
};
use uuid::Uuid;

use crate::{
    app::AppState,
    error::AppError,
    extractors::request_id::RequestId,
    middleware::auth_jwt::AuthUser,
    modules::todo::{
        filter_pagination::{FilterPaginationQuery, PaginationMeta},
        model::{CreateTodoRequest, Todo, UpdateTodoRequest},
        service::TodoService,
    },
    response::ApiResponse,
    utils::validation::validate_request,
};

pub async fn todos(
    State(state): State<AppState>,
    user: AuthUser,
    request_id: RequestId,
    Query(query): Query<FilterPaginationQuery>,
) -> Result<ApiResponse<Vec<Todo>, PaginationMeta>, AppError> {
    let (todos, meta) = TodoService::list_paginated(&state, user.user_id, query).await?;

    Ok(ApiResponse::with_meta(todos, meta, request_id.0))
}

pub async fn get_todo(
    State(state): State<AppState>,
    user: AuthUser,
    request_id: RequestId,
    Path(todo_id): Path<Uuid>,
) -> Result<ApiResponse<Todo>, AppError> {
    let todo = TodoService::get(&state, user.user_id, todo_id).await?;

    Ok(ApiResponse::new(todo, request_id.0))
}

pub async fn create_todo(
    State(state): State<AppState>,
    user: AuthUser,
    request_id: RequestId,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<ApiResponse<Todo>, AppError> {
    validate_request(&payload)?;

    let todo = TodoService::create(&state, user.user_id, payload).await?;

    Ok(ApiResponse::new(todo, request_id.0))
}

pub async fn update_todo(
    State(state): State<AppState>,
    user: AuthUser,
    Path(todo_id): Path<Uuid>,
    request_id: RequestId,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<ApiResponse<Todo>, AppError> {
    validate_request(&payload)?;

    let todo = TodoService::update(&state, user.user_id, todo_id, payload).await?;

    Ok(ApiResponse::new(todo, request_id.0))
}

pub async fn delete_todo(
    State(state): State<AppState>,
    user: AuthUser,
    Path(todo_id): Path<Uuid>,
) -> Result<(), AppError> {
    TodoService::delete(&state, user.user_id, todo_id).await?;

    Ok(())
}
