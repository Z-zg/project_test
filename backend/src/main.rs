use axum::{routing::{get, post}, Router};
use sqlx::{sqlite::SqlitePoolOptions};
use std::net::SocketAddr;
use config::{Config, ConfigError, File};
use serde::Deserialize;
use tower_http::cors::{CorsLayer, Any};

mod handlers;
mod models;

#[derive(Debug, Deserialize, Clone)]
struct Database {
    database_url: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Server {
    port: u16,
}

#[derive(Debug, Deserialize, Clone)]
struct AppConfig {
    server: Server,
    database: Database,
}

impl AppConfig {
    fn from_env() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("Config.toml"))
            .build()?;
        s.try_deserialize()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = AppConfig::from_env()?;

    // Initialize database pool
    let pool = SqlitePoolOptions::new()
        .connect(&config.database.database_url)
        .await?;

    // Run database migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    // CORS Layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any);

    // build our application with routes and state
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/register", post(handlers::register_user))
        .route("/login", post(handlers::login_user))
        .with_state(pool)
        .layer(cors);

    // run it with hyper on configured address and port
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}
