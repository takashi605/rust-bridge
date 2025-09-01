pub mod graphql;

use actix_web::{web, HttpResponse, Result};
use serde_json::json;
use sqlx::PgPool;
use crate::models::User;

pub async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({"status": "ok"})))
}

pub async fn get_users(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let users = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;
    
    Ok(HttpResponse::Ok().json(users))
}