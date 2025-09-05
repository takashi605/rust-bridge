#![recursion_limit = "512"]

mod handlers;

use actix_web::{web, App, HttpServer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting API server on http://0.0.0.0:8080");

    let schema = gql_schema_factory::create_schema().await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .route(
                "/graphql",
                web::post().to(handlers::graphql::graphql_handler),
            )
            .route("/", web::get().to(|| async { "API Server is running" }))
            .route("/health", web::get().to(handlers::health))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}

mod gql_schema_factory {
    use crate::repository_factory;
    use anyhow::Result;
    use api_schema::{build_schema_with_context, GrSchema};

    pub async fn create_schema() -> Result<GrSchema> {
        let user_repository = repository_factory::create_user_repository().await?;
        let schema = build_schema_with_context(user_repository);
        Ok(schema)
    }
}

mod repository_factory {
    use anyhow::Result;
    use repositories::{db, user::SqlxUserRepository};

    pub async fn create_user_repository() -> Result<SqlxUserRepository> {
        let pool = db::pool::create_connection_pool().await?;
        Ok(SqlxUserRepository::new(pool))
    }
}
