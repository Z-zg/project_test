use axum::{extract::{State, FromRef}, routing::{get, post}, Router};
use tower_http::cors::{Any, CorsLayer};
use std::{net::SocketAddr, sync::Arc};
use sqlx::migrate;
use tokio::net::TcpListener;

pub mod handlers;
pub mod models;
pub mod services;
mod config;
mod db;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::SqlitePool,
    pub config: Arc<config::AppConfig>,
}

impl FromRef<AppState> for sqlx::SqlitePool {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.db.clone()
    }
}

impl FromRef<AppState> for Arc<config::AppConfig> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.config.clone()
    }
}

async fn root() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    // Load configuration
    let config = match config::AppConfig::from_env() {
        Ok(config) => Arc::new(config),
        Err(e) => {
            eprintln!("Error loading configuration: {}", e);
            return;
        }
    };

    // Database setup
    let db_pool = match db::get_pool(&config.database.database_url).await {
        Ok(pool) => {
            println!("Connected to database successfully!");
            pool
        }
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            return;
        }
    };

    // Run migrations
    if let Err(e) = migrate!("./migrations").run(&db_pool).await {
        eprintln!("Failed to run migrations: {}", e);
        return;
    }

    let app_state = AppState {
        db: db_pool.clone(),
        config: config.clone(),
    };

    // CORS setup
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/register", post(handlers::auth_handler::register_user))
        .route("/login", post(handlers::auth_handler::login_user))
        .with_state(app_state)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    println!("listening on {}", addr);
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
