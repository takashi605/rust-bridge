pub mod graphql;

use actix_web::{HttpResponse, Result};
use serde_json::json;

pub async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({"status": "ok"})))
}
