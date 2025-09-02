use actix_web::{web, HttpResponse};
use juniper::{http::GraphQLRequest, ID};
use juniper_from_schema::graphql_schema_from_file;

graphql_schema_from_file!("../api_schema/user.graphql");

pub struct Context;
impl juniper::Context for Context {}

pub struct User {
    id: String,
    name: String,
    email: String,
}

impl UserFields for User {
    fn field_id(&self, _executor: &juniper::Executor<Context>) -> juniper::FieldResult<ID> {
        Ok(ID::from(self.id.clone()))
    }
    fn field_name(&self, _executor: &juniper::Executor<Context>) -> juniper::FieldResult<String> {
        Ok(self.name.clone())
    }
    fn field_email(&self, _executor: &juniper::Executor<Context>) -> juniper::FieldResult<String> {
        Ok(self.email.clone())
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_user(
        &self,
        _executor: &juniper::Executor<'_, Context>,
        _trail: &QueryTrail<'_, User, Walked>,
        id: juniper::ID,
    ) -> std::result::Result<User, juniper::FieldError> {
        Ok(User {
            id: id.to_string(),
            name: "Alice Johnson".to_string(),
            email: "alice@example.com".to_string(),
        })
    }
}

pub struct Mutation;
impl MutationFields for Mutation {
    fn field_noop(
        &self,
        _executor: &juniper::Executor<'_, Context>,
    ) -> std::result::Result<bool, juniper::FieldError> {
        Ok(true)
    }
}

pub async fn graphql_handler(
    schema: web::Data<Schema>,
    request: web::Json<GraphQLRequest>,
) -> HttpResponse {
    let ctx = Context;
    let response = request.execute(schema.get_ref(), &ctx);
    HttpResponse::Ok().json(response)
}
