use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../schema/user.graphql",
    query_path = "src/graphql/queries/fetch_user.graphql",
    response_derives = "Debug"
)]
pub struct Users;

#[tokio::test]
async fn test_fetch_user() {
    use users::Variables;

    let client = reqwest::Client::new();
    let endpoint = "http://api:8080/graphql";

    let resp =
        graphql_client::reqwest::post_graphql::<Users, _>(&client, endpoint, Variables {})
            .await
            .expect("GraphQL リクエストの送信に失敗しました");
    
    let user = resp.data.unwrap().user;
    assert_eq!(
        user.name, "Alice Johnson",
        "ユーザーの名前が期待と異なります"
    );

    assert_eq!(
        user.email, "alice@example.com",
        "ユーザーのメールアドレスが期待と異なります"
    );
}
