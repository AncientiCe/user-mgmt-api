use crate::models::{Claims, User};
use crate::config::Config;
use crate::error::{Result, UserError};
use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Header, Validation};
use time::{OffsetDateTime, Duration};

pub fn create_jwt(user: &User, config: &Config) -> Result<(String, OffsetDateTime)> {
    let now = OffsetDateTime::now_utc();
    let expires_at = now + Duration::hours(config.jwt_expiry_hours);

    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        exp: expires_at.unix_timestamp(),
        iat: now.unix_timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )?;

    Ok((token, expires_at))
}

pub fn verify_jwt(token: &str, config: &Config) -> Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &Validation::default(),
    )?;

    let now = OffsetDateTime::now_utc().unix_timestamp();
    if token_data.claims.exp < now {
        return Err(UserError::TokenExpired);
    }

    Ok(token_data.claims)
}
