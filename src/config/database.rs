use std::env;

use sqlx::{PgPool, Pool, Postgres, Sqlite, SqlitePool};

use crate::ServerError;

/// 初始化连接
pub async fn init_data_pool() -> Result<(Pool<Sqlite>, Pool<Postgres>), ServerError> {
    let sqlite_url = env::var("SQLITE_DATABASE_URL").unwrap();
    let postgres_url = env::var("POSTGRESQL_DATABASE_URL").unwrap();
    let sqlite_pool = SqlitePool::connect(&sqlite_url)
        .await
        .map_err(|_| ServerError::Message("连接sqlite错误".into()))?;
    let pool = PgPool::connect(&postgres_url)
        .await
        .map_err(|_| ServerError::Message("连接postgresql错误".into()))?;
    Ok((sqlite_pool,pool))
}
