use sqlx::{SqlitePool, Error};

pub async fn get_pool(database_url: &str) -> Result<SqlitePool, Error> {
    SqlitePool::connect(database_url).await
} 