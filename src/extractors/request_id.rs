use axum::{extract::FromRequestParts, http::request::Parts};
use uuid::Uuid;

pub struct RequestId(pub Uuid);

impl<S> FromRequestParts<S> for RequestId
where
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let request_id = parts
            .extensions
            .get::<Uuid>()
            .cloned()
            .unwrap_or_else(Uuid::new_v4);

        Ok(RequestId(request_id))
    }
}
