use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use argon2::{password_hash::{SaltString, PasswordHash, Encoding}, Argon2, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;
use sqlx::SqlitePool;

use crate::models::{AuthResponse, RegisterUser, User};

pub async fn register_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<RegisterUser>,
) -> impl IntoResponse {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = match argon2.hash_password(payload.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(e) => {
            eprintln!("Error hashing password: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(AuthResponse { message: "Error hashing password".to_string() })).into_response();
        }
    };

    let email = payload.email.to_lowercase();

    match sqlx::query_as::<_, User>(
        "INSERT INTO users (email, password_hash) VALUES (?, ?) RETURNING id, email, password_hash"
    )
    .bind(email)
    .bind(password_hash)
    .fetch_one(&pool)
    .await
    {
        Ok(_) => (
            StatusCode::CREATED,
            Json(AuthResponse { message: "User registered successfully".to_string() }),
        )
            .into_response(),
        Err(sqlx::Error::Database(dbe)) if dbe.is_unique_violation() => (
            StatusCode::CONFLICT,
            Json(AuthResponse { message: "Email already registered".to_string() }),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Error inserting user: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthResponse { message: "Internal server error".to_string() }),
            )
                .into_response()
        }
    }
}

pub async fn login_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<RegisterUser>, // Reusing RegisterUser for simplicity for now
) -> impl IntoResponse {
    let email = payload.email.to_lowercase();

    let user = sqlx::query_as::<_, User>("SELECT id, email, password_hash FROM users WHERE email = ?")
        .bind(email)
        .fetch_optional(&pool)
        .await;

    match user {
        Ok(Some(user)) => {
            let parsed_hash = match PasswordHash::parse(&user.password_hash, Encoding::default()) {
                Ok(hash) => hash,
                Err(e) => {
                    eprintln!("Error parsing password hash: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(AuthResponse { message: "Internal server error".to_string() })).into_response();
                }
            };

            let argon2 = Argon2::default();

            if argon2.verify_password(payload.password.as_bytes(), &parsed_hash).is_ok() {
                (
                    StatusCode::OK,
                    Json(AuthResponse { message: "Login successful!".to_string() }),
                )
                    .into_response()
            } else {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(AuthResponse { message: "Invalid credentials".to_string() }),
                )
                    .into_response()
            }
        }
        Ok(None) => (
            StatusCode::UNAUTHORIZED,
            Json(AuthResponse { message: "Invalid credentials".to_string() }),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Error fetching user: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthResponse { message: "Internal server error".to_string() }),
            )
                .into_response()
        }
    }
} 