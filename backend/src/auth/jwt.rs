use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct RegisteredUserSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}

impl TokenClaims {

    pub fn new(sub: String) -> TokenClaims {
        let now = chrono::Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;

        TokenClaims {
            sub: sub,
            exp: exp,
            iat: iat
        }
    }

    pub fn encode(self, header: &Header, key: &EncodingKey) -> String {
        encode(header, &self, key).unwrap_or_default()
    }
}