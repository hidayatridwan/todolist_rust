use uuid::Uuid;

use crate::{
    app::AppState,
    error::AppError,
    modules::todo::{
        filter_pagination::{FilterPaginationQuery, PaginationMeta},
        model::{CreateTodoRequest, Todo, UpdateTodoRequest},
        repository::TodoRepository,
    },
};

pub struct TodoService;

impl TodoService {
    pub async fn list_paginated(
        state: &AppState,
        user_id: Uuid,
        query: FilterPaginationQuery,
    ) -> Result<(Vec<Todo>, PaginationMeta), AppError> {
        let (page, limit) = query.normalize();

        let completed = query.parse_completed();

        let sort_field = query.sort_field();
        let sort_order = query.sort_order();

        let (todos, total_records) = TodoRepository::find_paginated(
            &state.db,
            user_id,
            page,
            limit,
            query.search_field,
            query.search_value,
            completed,
            sort_field,
            sort_order,
        )
        .await
        .map_err(|e| {
            tracing::error!("DB error: {:?}", e);
            AppError::InternalServerError
        })?;

        let total_pages = (total_records as f64 / limit as f64).ceil() as i64;

        Ok((
            todos,
            PaginationMeta {
                page,
                limit,
                total_records,
                total_pages,
            },
        ))
    }

    pub async fn get(state: &AppState, user_id: Uuid, todo_id: Uuid) -> Result<Todo, AppError> {
        TodoRepository::find_by_id(&state.db, user_id, todo_id)
            .await
            .map_err(|e| {
                tracing::error!("DB error: {:?}", e);
                AppError::InternalServerError
            })?
            .ok_or(AppError::NotFound("Todo not found".into()))
    }

    pub async fn create(
        state: &AppState,
        user_id: Uuid,
        payload: CreateTodoRequest,
    ) -> Result<Todo, AppError> {
        TodoRepository::create(
            &state.db,
            user_id,
            &payload.title,
            payload.description.as_deref(),
        )
        .await
        .map_err(|e| {
            tracing::error!("Create failed: {:?}", e);
            AppError::InternalServerError
        })
    }

    pub async fn update(
        state: &AppState,
        user_id: Uuid,
        todo_id: Uuid,
        payload: UpdateTodoRequest,
    ) -> Result<Todo, AppError> {
        TodoRepository::update(
            &state.db,
            user_id,
            todo_id,
            payload.title.as_deref(),
            payload.description.as_deref(),
            payload.completed,
        )
        .await
        .map_err(|e| {
            tracing::error!("Update failed: {:?}", e);
            AppError::InternalServerError
        })?
        .ok_or(AppError::NotFound("Todo not found".into()))
    }

    pub async fn delete(state: &AppState, user_id: Uuid, todo_id: Uuid) -> Result<(), AppError> {
        let deleted = TodoRepository::delete(&state.db, user_id, todo_id)
            .await
            .map_err(|e| {
                tracing::error!("Delete failed: {:?}", e);
                AppError::InternalServerError
            })?;

        if !deleted {
            return Err(AppError::NotFound("Todo not found".into()));
        }

        Ok(())
    }
}
