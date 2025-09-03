use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, SimpleObject, ID};
use anyhow::Result;
use repositories::mock::user::UserRepositoryMock;
use repositories::user::UserRepository;

#[derive(SimpleObject)]
struct User {
    id: String,
    name: String,
    email: String,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user(&self, _ctx: &Context<'_>, id: ID) -> Result<User> {
        let user_id = id.parse::<i32>()
            .map_err(|_| anyhow::anyhow!("Invalid user ID format"))?;
        
        let repository = UserRepositoryMock;
        let user = repository.fetch_by_id(user_id).unwrap();
        Ok(User {
            id: user.id.to_string(),
            name: user.name,
            email: user.email,
        })
    }
}

pub type GrSchema = async_graphql::Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema() -> GrSchema {
    async_graphql::Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}

pub fn schema_sdl() -> String {
    let schema = build_schema();
    schema.sdl()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdl() {
        let schema = schema_sdl();

        // 重要な型定義の存在を確認
        assert!(schema.contains("type QueryRoot {"));
        assert!(schema.contains("user(id: ID!): User!"));

        assert!(schema.contains("type User {"));
        assert!(schema.contains("id: String!"));
        assert!(schema.contains("name: String!"));
        assert!(schema.contains("email: String!"));

        assert!(schema.contains("schema {"));
        assert!(schema.contains("query: QueryRoot"));
    }

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
}
