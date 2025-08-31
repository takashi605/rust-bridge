use actix_web::{web, App, HttpServer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting API server on http://0.0.0.0:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "API Server is running" }))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}