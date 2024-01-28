use crate::models::{auth_error::AuthError, claims::Claims};

pub async fn protected(claims: Claims) -> Result<String, AuthError> {
    Ok(format!("{}", claims))
}
