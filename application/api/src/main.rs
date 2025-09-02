#![recursion_limit = "512"]

mod db;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer};
use api_schema::build_schema;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting API server on http://0.0.0.0:8080");

    // データベース接続プールを作成
    let pool = db::create_connection_pool().await?;

    let schema = build_schema();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(schema.clone()))
            .route(
                "/graphql",
                web::post().to(handlers::graphql::graphql_handler),
            )
            .route("/", web::get().to(|| async { "API Server is running" }))
            .route("/health", web::get().to(handlers::health))
            .route("/users", web::get().to(handlers::get_users))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}
