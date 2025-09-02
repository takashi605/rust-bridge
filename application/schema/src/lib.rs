use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, SimpleObject};

pub fn test() -> i32 {
    1
}

#[derive(SimpleObject)]
struct User {
    id: String,
    name: String,
    email: String,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user(&self, _ctx: &Context<'_>) -> User {
        User {
            id: "1".to_string(),
            name: "Alice Johnson".to_string(),
            email: "alice@example.com".to_string(),
        }
    }
}

type GrSchema = async_graphql::Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema() -> GrSchema {
    async_graphql::Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        test();
        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn test_fetch_user_query() {
        let query = r#"
            query {
                user {
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
}
