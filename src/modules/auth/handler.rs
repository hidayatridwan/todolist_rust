use axum::{Json, extract::State};

use crate::{
    app::AppState,
    error::AppError,
    modules::auth::{
        model::{AuthResponse, LoginRequest, RefreshRequest, RegisterRequest},
        service::AuthService,
    },
    utils::validation::validate_request,
};

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    validate_request(&payload)?;

    let res = AuthService::register(&state, payload).await?;

    Ok(Json(res))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let res = AuthService::login(&state, payload).await?;

    Ok(Json(res))
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let res = AuthService::refresh_token(&state, payload.refresh_token).await?;

    Ok(Json(res))
}

pub async fn logout(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<(), AppError> {
    AuthService::logout(&state, payload.refresh_token).await?;

    Ok(())
}
