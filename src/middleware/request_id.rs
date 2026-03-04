use axum::{
    body::Body,
    http::{HeaderValue, Request},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

pub async fn request_id_middleware(mut req: Request<Body>, next: Next) -> Response {
    let request_id = Uuid::new_v4();

    req.extensions_mut().insert(request_id);

    let mut res = next.run(req).await;

    res.headers_mut().insert(
        "x-request-id",
        HeaderValue::from_str(&request_id.to_string()).unwrap(),
    );

    res
}
