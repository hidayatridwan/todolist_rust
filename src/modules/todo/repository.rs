use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::todo::model::Todo;

pub struct TodoRepository;

impl TodoRepository {
    pub async fn find_paginated(
        db: &PgPool,
        user_id: Uuid,
        page: i64,
        limit: i64,
        search_field: Option<String>,
        search_value: Option<String>,
        completed: Option<Vec<bool>>,
    ) -> Result<(Vec<Todo>, i64), sqlx::Error> {
        let offset = (page - 1) * limit;

        let mut query_builder = sqlx::QueryBuilder::new("SELECT * FROM todos WHERE user_id = ");

        query_builder.push_bind(user_id);

        // search
        if let (Some(field), Some(value)) = (&search_field, &search_value) {
            if field == "title" {
                query_builder.push(" AND title ILIKE ");
                query_builder.push_bind(format!("%{}%", value));
            } else if field == "description" {
                query_builder.push(" AND description ILIKE ");
                query_builder.push_bind(format!("%{}%", value));
            }
        }

        // completed
        if let Some(values) = &completed {
            if !values.is_empty() && values.len() < 2 {
                query_builder.push(" AND completed = ");
                query_builder.push_bind(values[0]);
            }
        }

        query_builder.push(" ORDER BY created_at DESC ");
        query_builder.push(" LIMIT ");
        query_builder.push_bind(limit);
        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset);

        let todos = query_builder.build_query_as::<Todo>().fetch_all(db).await?;

        let mut count_builder =
            sqlx::QueryBuilder::new("SELECT COUNT(*) FROM todos WHERE user_id = ");

        count_builder.push_bind(user_id);

        // search
        if let (Some(field), Some(value)) = (&search_field, &search_value) {
            if field == "title" {
                count_builder.push(" AND title ILIKE ");
                count_builder.push_bind(format!("%{}%", value));
            } else if field == "description" {
                count_builder.push(" AND description ILIKE ");
                count_builder.push_bind(format!("%{}%", value));
            }
        }

        // completed
        if let Some(values) = &completed {
            if !values.is_empty() && values.len() < 2 {
                count_builder.push(" AND completed = ");
                count_builder.push_bind(values[0]);
            }
        }

        let total: (i64,) = count_builder.build_query_as().fetch_one(db).await?;

        Ok((todos, total.0))
    }

    pub async fn find_by_id(
        db: &PgPool,
        user_id: Uuid,
        todo_id: Uuid,
    ) -> Result<Option<Todo>, sqlx::Error> {
        sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1 AND user_id = $2")
            .bind(todo_id)
            .bind(user_id)
            .fetch_optional(db)
            .await
    }

    pub async fn create(
        db: &PgPool,
        user_id: Uuid,
        title: &str,
        description: Option<&str>,
    ) -> Result<Todo, sqlx::Error> {
        sqlx::query_as::<_, Todo>(
            "INSERT INTO todos (user_id, title, description)
             VALUES ($1, $2, $3)
             RETURNING *",
        )
        .bind(user_id)
        .bind(title)
        .bind(description)
        .fetch_one(db)
        .await
    }

    pub async fn update(
        db: &PgPool,
        user_id: Uuid,
        todo_id: Uuid,
        title: Option<&str>,
        description: Option<&str>,
        completed: Option<bool>,
    ) -> Result<Option<Todo>, sqlx::Error> {
        sqlx::query_as::<_, Todo>(
            r#"
            UPDATE todos
            SET
              title = COALESCE($3, title),
              description = COALESCE($4, description),
              completed = COALESCE($5, completed),
              updated_at = now()
            WHERE id = $1 AND user_id = $2
            RETURNING *
            "#,
        )
        .bind(todo_id)
        .bind(user_id)
        .bind(title)
        .bind(description)
        .bind(completed)
        .fetch_optional(db)
        .await
    }

    pub async fn delete(db: &PgPool, user_id: Uuid, todo_id: Uuid) -> Result<bool, sqlx::Error> {
        let res = sqlx::query("DELETE FROM todos WHERE id = $1 AND user_id = $2")
            .bind(todo_id)
            .bind(user_id)
            .execute(db)
            .await?;

        Ok(res.rows_affected() > 0)
    }
}
