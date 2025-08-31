use actix_web::{web, App, HttpServer, HttpResponse, Result};
use serde_json::json;

async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({"status": "ok"})))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting API server on http://0.0.0.0:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "API Server is running" }))
            .route("/health", web::get().to(health))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}