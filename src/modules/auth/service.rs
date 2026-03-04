use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::{
    app::AppState,
    error::AppError,
    modules::auth::{
        model::{AuthResponse, LoginRequest, RegisterRequest},
        repository::AuthRepository,
    },
    utils::{
        jwt::{generate_access_token, generate_refresh_token, verify_refresh_token},
        password::{hash_password, verify_password},
    },
};

pub struct AuthService;

impl AuthService {
    pub async fn register(
        state: &AppState,
        payload: RegisterRequest,
    ) -> Result<AuthResponse, AppError> {
        if AuthRepository::find_by_email(&state.db, &payload.email)
            .await
            .map_err(|e| {
                tracing::error!("DB error: {:?}", e);
                AppError::InternalServerError
            })?
            .is_some()
        {
            return Err(AppError::Conflict("Email already exists".into()));
        }

        let password_hash = hash_password(&payload.password).map_err(|e| {
            tracing::error!("Hash error: {:?}", e);
            AppError::InternalServerError
        })?;

        let user = AuthRepository::create_user(&state.db, &payload.email, &password_hash)
            .await
            .map_err(|e| {
                tracing::error!("Create user failed: {:?}", e);
                AppError::InternalServerError
            })?;

        Self::issue_tokens(state, user.id).await
    }

    pub async fn login(state: &AppState, payload: LoginRequest) -> Result<AuthResponse, AppError> {
        let user = AuthRepository::find_by_email(&state.db, &payload.email)
            .await
            .map_err(|e| {
                tracing::error!("DB error: {:?}", e);
                AppError::InternalServerError
            })?
            .ok_or(AppError::Unauthorized("Invalid credentials".into()))?;

        let valid = verify_password(&payload.password, &user.password_hash).map_err(|e| {
            tracing::error!("Verify error: {:?}", e);
            AppError::InternalServerError
        })?;

        if !valid {
            return Err(AppError::Unauthorized("Invalid credentials".into()));
        }

        Self::issue_tokens(state, user.id).await
    }

    pub async fn issue_tokens(state: &AppState, user_id: Uuid) -> Result<AuthResponse, AppError> {
        let access_token = generate_access_token(user_id, &state.jwt).map_err(|e| {
            tracing::error!("JWT error: {:?}", e);
            AppError::InternalServerError
        })?;
        let refresh_token = generate_refresh_token(user_id, &state.jwt).map_err(|e| {
            tracing::error!("JWT error: {:?}", e);
            AppError::InternalServerError
        })?;

        // hash refresh token before save
        let refresh_hash = hash_password(&refresh_token).map_err(|e| {
            tracing::error!("Hash refresh error: {:?}", e);
            AppError::InternalServerError
        })?;

        let expires_at = Utc::now() + Duration::days(state.jwt.refresh_exp_days);

        AuthRepository::save_refresh_token(&state.db, user_id, &refresh_hash, expires_at)
            .await
            .map_err(|e| {
                tracing::error!("Save refresh token failed: {:?}", e);
                AppError::InternalServerError
            })?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
        })
    }

    pub async fn refresh_token(
        state: &AppState,
        refresh_token: String,
    ) -> Result<AuthResponse, AppError> {
        // 1. verify JWT refresh token
        let token = verify_refresh_token(&refresh_token, &state.jwt)
            .map_err(|_| AppError::Unauthorized("Invalid refresh token".into()))?;

        let user_id = token.sub;

        // 2. ambil semua refresh token user
        let tokens = AuthRepository::find_valid_refresh_token(&state.db, user_id)
            .await
            .map_err(|e| {
                tracing::error!("DB error: {:?}", e);
                AppError::InternalServerError
            })?;

        // 3. cari token yang cocok (hash compare)
        let mut matched = None;

        for (id, token_hash, expires_at, revoked) in tokens {
            if revoked || expires_at < Utc::now() {
                continue;
            }

            if verify_password(&refresh_token, &token_hash).unwrap_or(false) {
                matched = Some(id);
                break;
            }
        }

        let token_id = matched.ok_or(AppError::BadRequest("Refresh token not found".into()))?;

        // 4. revoke token lama (ROTATION)
        AuthRepository::revoke_refresh_token(&state.db, token_id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to revoke token: {:?}", e);
                AppError::InternalServerError
            })?;

        // 5. issue token baru
        Self::issue_tokens(state, user_id).await
    }

    pub async fn logout(state: &AppState, refresh_token: String) -> Result<(), AppError> {
        let claims = verify_refresh_token(&refresh_token, &state.jwt).map_err(|e| {
            tracing::error!("Invalid refresh token: {:?}", e);
            AppError::InternalServerError
        })?;

        let user_id = claims.sub;

        let tokens = AuthRepository::find_valid_refresh_token(&state.db, user_id)
            .await
            .map_err(|e| {
                tracing::error!("DB error: {:?}", e);
                AppError::InternalServerError
            })?;

        for (id, _, _, _) in tokens {
            let _ = AuthRepository::revoke_refresh_token(&state.db, id).await;
        }

        Ok(())
    }
}
