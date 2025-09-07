#[tokio::test]
async fn test_fetch_user() {
    use api_test::test_helpers::graphql::cynic_queries::fetch_user::{FetchUser, FetchUserVars};
    use cynic::{http::ReqwestExt, QueryBuilder};

    let operation = FetchUser::build(FetchUserVars {
        id: "1".into(),
    });

    let client = reqwest::Client::new();
    let resp: cynic::GraphQlResponse<FetchUser> = client
        .post("http://api:8080/graphql")
        .run_graphql(operation)
        .await
        .expect("Failed to execute GraphQL request");

    println!("Response: {:?}", resp);

    let data = resp.data.expect("No data in response");
    assert_eq!(data.user.id, "1");
    assert_eq!(data.user.name, "Alice Johnson");
    assert_eq!(data.user.email, "alice@example.com");
}
