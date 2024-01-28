use super::{auth_error::AuthError, keys::KEYS};
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    // issuer
    pub iss: String,
    // subject
    pub sub: String,
    // audience
    pub aud: String,
    // expiration time
    pub exp: u64,
}

// https://github.com/tokio-rs/axum/blob/79b94b9bd651402d12624ec6de6c31fa0ceccf96/examples/jwt/src/main.rs
#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| AuthError::InvalidToken)?;
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;
        Ok(token_data.claims)
    }
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "iss: {}\nsub: {}\naud: {}\nexp: {}",
            self.iss, self.sub, self.aud, self.exp
        )
    }
}
