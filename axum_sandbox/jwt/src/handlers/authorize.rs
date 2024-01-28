use crate::models::{auth_body::AuthBody, auth_error::AuthError, claims::Claims, keys::KEYS};
use axum::Json;
use jsonwebtoken::{encode, Header};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn authorize() -> Result<Json<AuthBody>, AuthError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let exp = now + 3600;
    let claims = Claims {
        iss: "iss".to_string(),
        sub: "sub".to_string(),
        aud: "aud".to_string(),
        exp,
    };
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        // TODO: map_err()
        .unwrap();
    Ok(Json(AuthBody(token)))
}
