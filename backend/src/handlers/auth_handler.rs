use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sqlx::SqlitePool;

use crate::{models::user::{AuthResponse, RegisterUser}, services::auth_service};

pub async fn register_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<RegisterUser>,
) -> impl IntoResponse {
    match auth_service::register_user_service(&pool, payload).await {
        Ok(response) => (StatusCode::CREATED, Json(response)).into_response(),
        Err(e) => {
            if e == "Email already registered" {
                (StatusCode::CONFLICT, Json(AuthResponse { message: e })).into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(AuthResponse { message: e })).into_response()
            }
        }
    }
}

pub async fn login_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<RegisterUser>,
) -> impl IntoResponse {
    match auth_service::login_user_service(&pool, payload).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (StatusCode::UNAUTHORIZED, Json(AuthResponse { message: e })).into_response(),
    }
} 