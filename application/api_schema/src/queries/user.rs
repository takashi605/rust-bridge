use async_graphql::{Context, Object, SimpleObject, ID};
use anyhow::Result;
use repositories::mock::user::UserRepositoryMock;
use repositories::user::UserRepository;

#[derive(SimpleObject)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

pub struct UserQuery;

#[Object]
impl UserQuery {
    pub async fn user(&self, _ctx: &Context<'_>, id: ID) -> Result<User> {
        let user_id = id.parse::<i32>()
            .map_err(|_| anyhow::anyhow!("Invalid user ID format"))?;
        
        let repository = UserRepositoryMock;
        let user = repository.fetch_by_id(user_id)?;
        Ok(User {
            id: user.id.to_string(),
            name: user.name,
            email: user.email,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::build_schema;

    #[tokio::test]
    async fn test_fetch_user_query() {
        let query = r#"
            query {
                user (id: "1") {
                    id
                    name
                    email
                }
            }
        "#;

        let schema = build_schema();
        let resp = schema.execute(query).await;

        let respond_json = resp.data.into_json().unwrap();
        let expected_json = serde_json::json!({
            "user": {
                "id": "1",
                "name": "Alice Johnson",
                "email": "alice@example.com"
            }
        });

        assert_eq!(respond_json, expected_json);
    }

    #[tokio::test]
    async fn test_user_query_with_invalid_id() {
        let query = r#"
            query {
                user (id: "invalid") {
                    id
                    name
                    email
                }
            }
        "#;

        let schema = build_schema();
        let resp = schema.execute(query).await;

        assert!(resp.errors.len() > 0);
        assert!(resp.errors[0].message.contains("Invalid user ID format"));
    }

    #[tokio::test]
    async fn test_user_query_with_nonexistent_id() {
        let query = r#"
            query {
                user (id: "999") {
                    id
                    name
                    email
                }
            }
        "#;

        let schema = build_schema();
        let resp = schema.execute(query).await;

        assert!(resp.errors.len() > 0);
        assert!(resp.errors[0].message.contains("User not found"));
    }
}