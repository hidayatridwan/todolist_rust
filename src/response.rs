use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct ApiResponse<T, M = ()> {
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<M>,
    pub request_id: Uuid,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T, request_id: Uuid) -> Self {
        Self {
            data,
            meta: None,
            request_id,
        }
    }
}

impl<T, M> ApiResponse<T, M> {
    pub fn with_meta(data: T, meta: M, request_id: Uuid) -> Self {
        Self {
            data,
            meta: Some(meta),
            request_id,
        }
    }
}

impl<T, M> IntoResponse for ApiResponse<T, M>
where
    T: Serialize,
    M: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
