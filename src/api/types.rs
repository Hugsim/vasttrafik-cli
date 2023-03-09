use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub scope: String,
    pub token_type: String,
    pub expires_in: i32,
    pub access_token: String,
}

#[derive(Debug)]
pub struct Token {
    pub token: String,
}

impl Token {
    pub fn new(token: String) -> Token {
        Token { token }
    }
}
