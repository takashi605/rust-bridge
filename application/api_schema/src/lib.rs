use anyhow::Result;
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, SchemaBuilder, ID};

mod queries;
use queries::user::{User, UserQuery};

pub struct QueryRoot {
    user_query: UserQuery,
}

impl QueryRoot {
    pub fn new() -> Self {
        Self {
            user_query: UserQuery,
        }
    }
}

#[Object]
impl QueryRoot {
    async fn user(&self, ctx: &Context<'_>, id: ID) -> Result<User> {
        self.user_query.user(ctx, id).await
    }
}

pub type GrSchema = async_graphql::Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema_with_context<R: 'static + Send + Sync>(repository: R) -> GrSchema
where
    R: repositories::user::UserRepository,
{
    let schema = initialize_schema_builder().data(repository).finish();
    schema
}

pub fn build_schema() -> GrSchema {
    initialize_schema_builder().finish()
}

fn initialize_schema_builder() -> SchemaBuilder<QueryRoot, EmptyMutation, EmptySubscription> {
    async_graphql::Schema::build(QueryRoot::new(), EmptyMutation, EmptySubscription)
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
}
