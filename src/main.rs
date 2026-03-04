use std::net::SocketAddr;

use axum::serve;
use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, fmt};

use crate::{
    app::{AppState, create_app},
    config::{env::Env, jwt::JwtConfig},
    db::postgres::create_pg_pool,
};

mod app;
mod config;
mod db;
mod error;
mod extractors;
mod middleware;
mod response;
mod utils;
mod modules {
    pub mod auth;
    pub mod todo;
}

#[tokio::main]
async fn main() {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let env = Env::from_env();

    let db = create_pg_pool(&env.database_url).await;

    let jwt = JwtConfig::new(
        env.jwt_secret.clone(),
        env.jwt_access_expire_minutes,
        env.jwt_refresh_expire_days,
    );

    let state = AppState { db, jwt };

    let app = create_app(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], env.app_port));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 {} running on {}", env.app_name, addr);

    serve(listener, app).await.unwrap();
}
