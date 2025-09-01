mod db;
mod models;
mod handlers;

use actix_web::{web, App, HttpServer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting API server on http://0.0.0.0:8080");

    // データベース接続プールを作成
    let pool = db::create_connection_pool().await?;
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(|| async { "API Server is running" }))
            .route("/health", web::get().to(handlers::health))
            .route("/users", web::get().to(handlers::get_users))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}