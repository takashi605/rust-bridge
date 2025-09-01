use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}