use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(sqlx::FromRow, Serialize)]
pub struct Todo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Validate)]
pub struct CreateTodoRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateTodoRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: Option<String>,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    pub completed: Option<bool>,
}
