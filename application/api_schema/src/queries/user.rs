use std::sync::Arc;

use async_graphql::{Context, Object, SimpleObject, ID};
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
    pub async fn user(&self, ctx: &Context<'_>, id: ID) -> async_graphql::Result<User> {
        let user_id = id
            .parse::<i32>()
            .map_err(|_| async_graphql::Error::new("Invalid user ID format"))?;

        let repository = ctx
            .data::<Arc<dyn UserRepository>>()
            .map_err(|_| async_graphql::Error::new("UserRepository not found in context"))?;

        let user = repository
            .fetch_by_id(user_id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(User {
            id: user.id.to_string(),
            name: user.name,
            email: user.email,
        })
    }
}

#[cfg(test)]
mod tests {
    use repositories::mock::user::UserRepositoryMock;

    use crate::build_schema_with_context;

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

        let repo = UserRepositoryMock;
        let schema = build_schema_with_context(repo);
        let resp = schema.execute(query).await;

        let respond_json = resp
            .data
            .into_json()
            .expect("Failed to convert response data to JSON");
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

        let repo = UserRepositoryMock;
        let schema = build_schema_with_context(repo);
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

        let repo = UserRepositoryMock;
        let schema = build_schema_with_context(repo);
        let resp = schema.execute(query).await;

        assert!(resp.errors.len() > 0);
        assert!(resp.errors[0]
            .message
            .contains("User with ID 999 not found"));
    }
}
