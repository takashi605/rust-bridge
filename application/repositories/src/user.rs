use crate::db::models::User;

pub struct UserRepository;
impl UserRepository {
    pub fn fetch_by_id(id: i32) -> Result<User, &'static str> {
        // TODO DBからユーザ情報を取得する処理を実装
        // モックデータ返却
        if id == 1 {
            Ok(User {
                id: 1,
                name: "Alice Johnson".to_string(),
                email: "alice@example.com".to_string(),
            })
        } else {
            Err("User not found")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_single_user() {
        let user = UserRepository::fetch_by_id(1).unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Alice Johnson");
        assert_eq!(user.email, "alice@example.com");
    }
}