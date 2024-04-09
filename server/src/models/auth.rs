use serde::{Deserialize, Serialize};

use utoipa::{ToResponse, ToSchema};

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema, ToResponse, Debug)]
pub struct TokenResponse {
    pub token_type: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
}

impl TokenResponse {
    pub fn new(token: String) -> Self {
        TokenResponse {
            token_type: "jwt".to_string(),
            access_token: token,
            refresh_token: None,
        }
    }
}
