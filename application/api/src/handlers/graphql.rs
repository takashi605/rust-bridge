pub mod queries {
    pub mod user;
}
pub mod mutations {
    pub mod notice;
}

use actix_web::web;
use crate::schema::GrSchema;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

pub async fn graphql_handler(schema: web::Data<GrSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
