use serde::{Deserialize, Serialize};

use utoipa::{ToResponse, ToSchema};
use uuid::Uuid;

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema, ToResponse, Debug)]
pub struct RegisterResponse {
    pub user_id: Uuid,
}

#[derive(Deserialize, Serialize, ToSchema, ToResponse, Debug)]
pub struct TokenResponse {
    pub token_type: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: i64,
}

impl TokenResponse {
    pub fn new(token: String, expires_in: i64) -> Self {
        TokenResponse {
            token_type: "jwt".to_string(),
            access_token: token,
            refresh_token: None,
            expires_in,
        }
    }
}
