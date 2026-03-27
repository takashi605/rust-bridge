use std::sync::Arc;

use async_graphql::{Context, EmptySubscription, Object, SchemaBuilder, ID};

use crate::handlers::graphql::queries::user::{User, UserQuery};
use crate::handlers::graphql::mutations::notice::{NoticeMutation, NoticeInput, SendEmailPayload};

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
    async fn user(&self, ctx: &Context<'_>, id: ID) -> async_graphql::Result<User> {
        self.user_query.user(ctx, id).await
    }
}

pub struct MutationRoot {
    notice_mutation: NoticeMutation,
}

impl MutationRoot {
    pub fn new() -> Self {
        Self {
            notice_mutation: NoticeMutation,
        }
    }
}

#[Object]
impl MutationRoot {
    async fn notify(&self, ctx: &Context<'_>, recipient: String, notice: NoticeInput) -> async_graphql::Result<SendEmailPayload> {
        self.notice_mutation.notify(ctx, recipient, notice).await
    }
}

pub type GrSchema = async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn build_schema_with_context<R: 'static + Send + Sync>(repository: R) -> GrSchema
where
    R: repositories::user::UserRepository,
{
    let repository: Arc<dyn repositories::user::UserRepository> = Arc::new(repository);
    let schema = initialize_schema_builder().data(repository).finish();
    schema
}

pub fn build_schema() -> GrSchema {
    initialize_schema_builder().finish()
}

fn initialize_schema_builder() -> SchemaBuilder<QueryRoot, MutationRoot, EmptySubscription> {
    async_graphql::Schema::build(QueryRoot::new(), MutationRoot::new(), EmptySubscription)
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
