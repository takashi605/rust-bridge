// https://countries.trevorblades.com に対して GraphQL クエリを送信するテスト
// ライブラリや複雑な構造体を使わずに、シンプルな構造でテストを行う

#[tokio::test]
async fn simple_test_graphql_query_countries_name() {
    let client = reqwest::Client::new();

    let gql_query = r#"
        query {
            countries {
                name
            }
        }
        "#;

    let json_body = serde_json::json!({
        "query": gql_query
    });

    let req = client
        .post("https://countries.trevorblades.com")
        .json(&json_body);

    let resp = req.send().await.expect("リクエストの送信に失敗しました");

    let resp_json: response::GraphQlRespWrapper<response::QueryResponse> = resp
        .json()
        .await
        .expect("レスポンスの JSON 解析に失敗しました");

    assert_eq!(
        resp_json.data.countries[0].name, "Andorra",
        "最初の国の名前が期待と異なります"
    );
    eprintln!("最初の国の名前: {}", resp_json.data.countries[0].name);
}

// レスポンスの構造体定義
mod response {
    // すべてのレスポンスに共通するラッパー構造体
    #[derive(serde::Deserialize, Debug)]
    pub struct GraphQlRespWrapper<T> {
        pub data: T,
    }

    /*
    + 以下、利用APIに依存する詳細なレスポンス構造体
    */

    #[derive(serde::Deserialize, Debug)]
    pub struct QueryResponse {
        pub countries: Vec<Country>,
    }

    // 国の情報を表す構造体
    #[derive(serde::Deserialize, Debug)]
    pub struct Country {
        pub name: String,
    }
}
