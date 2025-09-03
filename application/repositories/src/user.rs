use crate::db::models::User;
use anyhow::Result;

pub trait UserRepository {
    fn fetch_by_id(id: i32) -> Result<User>;
}