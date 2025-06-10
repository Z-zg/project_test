use argon2::{password_hash::{SaltString, PasswordHash, Encoding}, Argon2, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;
use sqlx::SqlitePool;

use crate::models::user::{AuthResponse, RegisterUser, User};

pub async fn register_user_service(
    pool: &SqlitePool,
    payload: RegisterUser,
) -> Result<AuthResponse, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = match argon2.hash_password(payload.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(e) => {
            eprintln!("Error hashing password: {}", e);
            return Err("Error hashing password".to_string());
        }
    };

    let email = payload.email.to_lowercase();

    match sqlx::query_as::<_, User>(
        "INSERT INTO users (email, password_hash) VALUES (?, ?) RETURNING id, email, password_hash"
    )
    .bind(email)
    .bind(password_hash)
    .fetch_one(pool)
    .await
    {
        Ok(_) => Ok(AuthResponse { message: "User registered successfully".to_string() }),
        Err(sqlx::Error::Database(dbe)) if dbe.is_unique_violation() => {
            Err("Email already registered".to_string())
        }
        Err(e) => {
            eprintln!("Error inserting user: {}", e);
            Err("Internal server error".to_string())
        }
    }
}

pub async fn login_user_service(
    pool: &SqlitePool,
    payload: RegisterUser,
) -> Result<AuthResponse, String> {
    let email = payload.email.to_lowercase();

    let user = sqlx::query_as::<_, User>("SELECT id, email, password_hash FROM users WHERE email = ?")
        .bind(email)
        .fetch_optional(pool)
        .await;

    match user {
        Ok(Some(user)) => {
            let parsed_hash = match PasswordHash::parse(&user.password_hash, Encoding::default()) {
                Ok(hash) => hash,
                Err(e) => {
                    eprintln!("Error parsing password hash: {}", e);
                    return Err("Internal server error".to_string());
                }
            };

            let argon2 = Argon2::default();

            if argon2.verify_password(payload.password.as_bytes(), &parsed_hash).is_ok() {
                Ok(AuthResponse { message: "Login successful!".to_string() })
            } else {
                Err("Invalid credentials".to_string())
            }
        }
        Ok(None) => Err("Invalid credentials".to_string()),
        Err(e) => {
            eprintln!("Error fetching user: {}", e);
            Err("Internal server error".to_string())
        }
    }
} 