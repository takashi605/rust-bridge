#[cynic::schema("app")]
mod schema {}

#[derive(cynic::QueryFragment, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(cynic::QueryVariables)]
pub struct FetchUserVars {
    pub id: cynic::Id,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "FetchUserVars")]
pub struct FetchUser {
    #[arguments(id: $id)]
    pub user: User,
}
