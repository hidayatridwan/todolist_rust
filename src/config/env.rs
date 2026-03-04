use std::env;

#[derive(Clone)]
pub struct Env {
    pub app_name: String,
    pub app_port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_access_expire_minutes: i64,
    pub jwt_refresh_expire_days: i64,
}

impl Env {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            app_name: env::var("APP_NAME").expect("APP_NAME missing"),
            app_port: env::var("APP_PORT")
                .unwrap_or_else(|_| "3000".into())
                .parse()
                .expect("APP_PORT invalid"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL missing"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET missing"),
            jwt_access_expire_minutes: env::var("JWT_ACCESS_EXPIRE_MINUTES")
                .unwrap_or_else(|_| "15".into())
                .parse()
                .expect("JWT_ACCESS_EXPIRE_MINUTES invalid"),
            jwt_refresh_expire_days: env::var("JWT_REFRESH_EXPIRE_DAYS")
                .unwrap_or_else(|_| "7".into())
                .parse()
                .expect("JWT_REFRESH_EXPIRE_DAYS invalid"),
        }
    }
}
