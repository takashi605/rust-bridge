use actix_web::{web, App, HttpServer, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}

async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({"status": "ok"})))
}

async fn get_users(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let users = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;
    
    Ok(HttpResponse::Ok().json(users))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting API server on http://0.0.0.0:8080");

    // データベース接続プールを作成
    let database_url = "postgres://postgres:postgres@db:5432/graphql_api";
    let pool = PgPool::connect(database_url).await?;
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(|| async { "API Server is running" }))
            .route("/health", web::get().to(health))
            .route("/users", web::get().to(get_users))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}