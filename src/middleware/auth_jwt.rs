use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use uuid::Uuid;

use crate::{app::AppState, error::AppError, utils::jwt::verify_access_token};

pub struct AuthUser {
    pub user_id: Uuid,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or(AppError::Unauthorized(
                "Missing Authorization header".into(),
            ))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized(
                "Invalid Authorization header".into(),
            ))?;

        let claims = verify_access_token(token, &state.jwt)
            .map_err(|_| AppError::Unauthorized("Invalid or expired token".into()))?;

        Ok(AuthUser {
            user_id: claims.sub,
        })
    }
}
