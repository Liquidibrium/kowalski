use crate::entities::user::{AuthProvider, UserEntity, UserStatus};
use crate::errors::AppResponseError;
use crate::models::auth::{LoginRequest, RegisterRequest, RegisterResponse, TokenResponse};
use crate::repository::user::{create_user, get_user_by_email};
use crate::service::jwt::create_token;
use crate::state::AppState;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::Json;
use http::StatusCode;
use log::{error, info};

use anyhow::Context;
use chrono::Utc;
use std::sync::Arc;

use uuid::Uuid;

#[utoipa::path(post, path = "/api/auth/login", responses((status = StatusCode::OK, body = TokenResponse)))]
pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(login_request): Json<LoginRequest>,
) -> Response {
    let user = get_user_by_email(&state.db, &login_request.email).await;
    if let Err(e) = user {
        error!("Error getting user by email: {:?}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AppResponseError::new("Internal server error")),
        )
            .into_response();
    }
    let user = user.unwrap();
    if user.is_none() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(AppResponseError::new("Invalid email or password")),
        )
            .into_response();
    }
    let user = user.unwrap();
    if user.status.clone() != UserStatus::Active {
        return (
            StatusCode::UNAUTHORIZED,
            Json(AppResponseError::new("User is not active")),
        )
            .into_response();
    }
    let password = user.password.clone();
    if password.is_none() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(AppResponseError::new("Invalid email or password")),
        )
            .into_response();
    }

    if !bcrypt::verify(&login_request.password, &password.unwrap()).unwrap() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(AppResponseError::new("Invalid email or password")),
        )
            .into_response();
    }
    let token = create_token(
        &user,
        state.config.jwt_hmac_key.as_str(),
        state.config.jwt_expiration_time,
    )
    .unwrap();
    (StatusCode::OK, Json(TokenResponse::new(token, state.config.jwt_expiration_time))).into_response()
}

#[utoipa::path(post, path = "/api/auth/register", responses((status = StatusCode::CREATED, body = RegisterResponse)))]
pub async fn register_handler(
    State(state): State<Arc<AppState>>,
    Json(register_request): Json<RegisterRequest>,
) -> Response {
    let user = get_user_by_email(&state.db, &register_request.email).await;
    if let Err(e) = user {
        error!("Error getting user by email: {:?}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AppResponseError::new("Internal server error")),
        )
            .into_response();
    }
    let user = user.unwrap();
    if user.is_some() {
        return (
            StatusCode::CONFLICT,
            Json(AppResponseError::new("User already exists")),
        )
            .into_response();
    }
    let hashed_password = bcrypt::hash(register_request.password.as_str(), 12).unwrap();
    let user = UserEntity {
        id: Uuid::now_v7(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        first_name: register_request.first_name.clone(),
        last_name: register_request.last_name.clone(),
        email: register_request.email.clone(),
        password: Some(hashed_password),
        status: UserStatus::Pending,
        provider: AuthProvider::Local,
    };

    info!("Creating user: {:?}", user);
    let user = create_user(&state.db, user).await;
    if let Err(e) = user {
        error!("Error creating user: {:?}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AppResponseError::new("Internal server error")),
        )
            .into_response();
    }

    //TODO: send email to user with verification link
    let user = user.unwrap();
    let token = create_token(
        &user,
        state.config.jwt_hmac_key.as_str(),
        state.config.jwt_expiration_time,
    )
    .context("Error creating token");
    if let Err(e) = token {
        error!("Error creating token: {:?}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AppResponseError::new("Internal server error")),
        )
            .into_response();
    }
    (
        StatusCode::CREATED,
        Json(RegisterResponse {
            user_id: user.id.clone(),
        }),
    )
        .into_response()
}
