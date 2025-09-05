use sqlx::PgPool;
use anyhow::Result;

pub async fn create_connection_pool() -> Result<PgPool> {
    let database_url = "postgres://postgres:postgres@db:5432/graphql_api";
    let pool = PgPool::connect(database_url).await?;
    Ok(pool)
}