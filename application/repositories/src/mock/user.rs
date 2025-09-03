use crate::{db::models::User, user::UserRepository};
use anyhow::Result;

pub struct UserRepositoryMock;
impl UserRepository for UserRepositoryMock {
    fn fetch_by_id(&self, id: i32) -> Result<User> {
        if id == 1 {
            Ok(User {
                id: 1,
                name: "Alice Johnson".to_string(),
                email: "alice@example.com".to_string(),
            })
        } else {
            Err(anyhow::anyhow!("User not found"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_single_user() {
        let user = UserRepositoryMock.fetch_by_id(1).unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Alice Johnson");
        assert_eq!(user.email, "alice@example.com");
    }
}
