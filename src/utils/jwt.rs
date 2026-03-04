use chrono::{Duration, Utc};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::jwt::JwtConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
    pub typ: TokenType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TokenType {
    Access,
    Refresh,
}

pub fn generate_access_token(user_id: Uuid, config: &JwtConfig) -> Result<String, Error> {
    let now = Utc::now();
    let exp = now + Duration::minutes(config.access_exp_minutes);

    let claims = Claims {
        sub: user_id,
        iat: now.timestamp() as usize,
        exp: exp.timestamp() as usize,
        typ: TokenType::Access,
    };

    encode(
        &Header::new(config.algorithm),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
}

pub fn generate_refresh_token(user_id: Uuid, config: &JwtConfig) -> Result<String, Error> {
    let now = Utc::now();
    let exp = now + Duration::days(config.refresh_exp_days);

    let claims = Claims {
        sub: user_id,
        iat: now.timestamp() as usize,
        exp: exp.timestamp() as usize,
        typ: TokenType::Refresh,
    };

    encode(
        &Header::new(config.algorithm),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
}

pub fn verify_token(token: &str, config: &JwtConfig) -> Result<Claims, Error> {
    let validation = Validation::new(config.algorithm);

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.secret.as_bytes()),
        &validation,
    )?;

    Ok(token_data.claims)
}

pub fn verify_access_token(token: &str, config: &JwtConfig) -> Result<Claims, &'static str> {
    let claims = verify_token(token, config).map_err(|_| "Invalid token")?;

    if claims.typ != TokenType::Access {
        return Err("Invalid access token type");
    }

    Ok(claims)
}

pub fn verify_refresh_token(token: &str, config: &JwtConfig) -> Result<Claims, &'static str> {
    let claims = verify_token(token, config).map_err(|_| "Invalid token")?;

    if claims.typ != TokenType::Refresh {
        return Err("Invalid refresh token type");
    }

    Ok(claims)
}
