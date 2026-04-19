use chrono::Duration;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::OnceLock;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    Json,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: u64,
}

static JWT_SECRET_CACHE: OnceLock<String> = OnceLock::new();

/// Call this once at application startup. Panics if JWT_SECRET is not set.
pub fn init_jwt_secret() {
    let secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET environment variable must be set");
    JWT_SECRET_CACHE.set(secret).expect("JWT_SECRET already initialized");
}

fn jwt_secret() -> &'static str {
    JWT_SECRET_CACHE.get().expect("JWT_SECRET not initialized — call auth::init_jwt_secret() at startup")
}

pub fn create_token(user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp")
        .timestamp() as u64;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret().as_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )?;
    Ok(token_data.claims)
}

pub struct AuthUser(pub Uuid);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(serde_json::json!({"error": "Missing authorization header"})),
                )
            })?;

        let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Invalid authorization header format"})),
            )
        })?;

        let claims = validate_token(token).map_err(|e| {
            tracing::warn!("JWT validation failed: {}", e);
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Invalid or expired token"})),
            )
        })?;

        Ok(AuthUser(claims.sub))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() {
        JWT_SECRET_CACHE.get_or_init(|| "test_secret".to_string());
    }

    #[test]
    fn test_create_and_validate_token_roundtrip() {
        setup();
        let user_id = Uuid::new_v4();
        let token = create_token(user_id).expect("should create token");
        let claims = validate_token(&token).expect("should validate token");
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    fn test_validate_invalid_token_fails() {
        setup();
        let result = validate_token("not.a.token");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_tampered_token_fails() {
        setup();
        let user_id = Uuid::new_v4();
        let token = create_token(user_id).expect("should create token");
        let tampered = format!("{}x", token);
        let result = validate_token(&tampered);
        assert!(result.is_err());
    }
}
