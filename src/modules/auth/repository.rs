use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::auth::model::User;

pub struct AuthRepository;

impl AuthRepository {
    pub async fn find_by_email(db: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT id, email, password_hash FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(db)
            .await
    }

    pub async fn create_user(
        db: &PgPool,
        email: &str,
        password_hash: &str,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (email, password_hash)
             VALUES ($1, $2)
             RETURNING id, email, password_hash",
        )
        .bind(email)
        .bind(password_hash)
        .fetch_one(db)
        .await
    }

    pub async fn save_refresh_token(
        db: &PgPool,
        user_id: Uuid,
        token_hash: &str,
        expires_at: DateTime<chrono::Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
             VALUES ($1, $2, $3)",
        )
        .bind(user_id)
        .bind(token_hash)
        .bind(expires_at)
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn find_valid_refresh_token(
        db: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<(Uuid, String, DateTime<Utc>, bool)>, sqlx::Error> {
        sqlx::query_as(
            "SELECT id, token_hash, expires_at, revoked
         FROM refresh_tokens
         WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_all(db)
        .await
    }

    pub async fn revoke_refresh_token(db: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE refresh_tokens SET revoked = true WHERE id = $1")
            .bind(user_id)
            .execute(db)
            .await?;

        Ok(())
    }
}
