use jsonwebtoken::Algorithm;

#[derive(Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub access_exp_minutes: i64,
    pub refresh_exp_days: i64,
    pub algorithm: Algorithm,
}

impl JwtConfig {
    pub fn new(secret: String, access_exp_minutes: i64, refresh_exp_days: i64) -> Self {
        Self {
            secret,
            access_exp_minutes,
            refresh_exp_days,
            algorithm: Algorithm::HS256,
        }
    }
}
