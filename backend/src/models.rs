use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub message: String,
} 