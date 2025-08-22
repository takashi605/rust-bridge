// https://countries.trevorblades.com に対して GraphQL クエリを送信するテスト
// simple テストに比べ、リクエスト・レスポンスの構造体をスマートにしている

#[tokio::test]
async fn test_graphql_query_countries_name() {
    let gql_client = response::GraphQLClient::new()
        .target("https://countries.trevorblades.com")
        .query("query { countries { name } }");

    let req = gql_client.create_request_builder();

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
    pub struct GraphQLClient {
        reqwest_client: reqwest::Client,
        target: String,
        query: String,
    }
    impl GraphQLClient {
        pub fn new() -> Self {
            Self {
                reqwest_client: reqwest::Client::new(),
                target: String::new(),
                query: String::new(),
            }
        }

        pub fn target(mut self, target: &str) -> Self {
            self.target = target.to_string();
            self
        }

        pub fn query(mut self, query: &str) -> Self {
            self.query = query.to_string();
            self
        }

        pub fn create_request_builder(&self) -> reqwest::RequestBuilder {
            let json_body = serde_json::json!({
                "query": self.query
            });

            self.reqwest_client.post(&self.target).json(&json_body)
        }
    }

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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_create_request_builder() {
            let mut gql_client = GraphQLClient::new();
            gql_client = gql_client
                .target("https://countries.trevorblades.com")
                .query("query { countries { name } }");

            let req_builder: reqwest::RequestBuilder = gql_client.create_request_builder();

            let req = req_builder
                .build()
                .expect("リクエストのビルドに失敗しました");

            assert_eq!(req.method(), reqwest::Method::POST);
            assert_eq!(req.url().as_str(), "https://countries.trevorblades.com/");

            let body_bytes = req
                .body()
                .expect("リクエストボディが存在しません")
                .as_bytes()
                .expect("リクエストボディをバイト列として取得できません");

            let body_json: serde_json::Value = serde_json::from_slice(body_bytes)
                .expect("リクエストボディのJSON解析に失敗しました");

            assert_eq!(
                body_json["query"].as_str().unwrap(),
                "query { countries { name } }"
            );
        }
    }
}
