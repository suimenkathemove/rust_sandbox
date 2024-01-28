use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    // secretはサービスプロバイダで生成する
    let secret = std::env::var("SECRET").unwrap_or("secret".to_string());
    Keys::new(secret)
});

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: String) -> Self {
        let secret = secret.as_bytes();
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
