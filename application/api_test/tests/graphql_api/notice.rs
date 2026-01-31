#[tokio::test]
async fn test_notify_mutation() {
    use api_test::test_helpers::graphql::cynic_queries::notify::{
        NoticeInput, NotifyMutation, NotifyMutationVars,
    };
    use cynic::{http::ReqwestExt, MutationBuilder};

    let operation = NotifyMutation::build(NotifyMutationVars {
        recipient: "test@example.com".to_string(),
        notice: NoticeInput {
            subject: "Test Subject".to_string(),
            message: "Test Message".to_string(),
        },
    });

    let client = reqwest::Client::new();
    let resp: cynic::GraphQlResponse<NotifyMutation> = client
        .post("http://api:8080/graphql")
        .run_graphql(operation)
        .await
        .expect("Failed to execute GraphQL request");

    println!("Response: {:?}", resp);

    let data = resp.data.expect("No data in response");
    assert_eq!(data.notify.recipient, "test@example.com");
    assert_eq!(data.notify.notice.subject, "Test Subject");
    assert_eq!(data.notify.notice.message, "Test Message");

    // TODO: メールが実際に送信されたかどうかをMailHogのAPIなどで確認する
}
