use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/countries.graphql",
    response_derives = "Debug"
)]
pub struct Countries;

#[tokio::test]
async fn test_graphql_query_countries_name_with_graphql_client() {
    use countries::Variables;

    let client = reqwest::Client::new();
    let endpoint = "https://countries.trevorblades.com";

    let resp =
        graphql_client::reqwest::post_graphql::<Countries, _>(&client, endpoint, Variables {})
            .await
            .expect("GraphQL リクエストの送信に失敗しました");

    let countries = resp.data.unwrap().countries;
    assert_eq!(
        countries[0].name, "Andorra",
        "最初の国の名前が期待と異なります"
    );
}
