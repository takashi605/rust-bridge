#[tokio::test]
async fn test_notify_mutation() {
    use api_test::test_helpers::graphql::cynic_queries::notify::{
        NoticeInput, NotifyMutation, NotifyMutationVars,
    };
    use cynic::{http::ReqwestExt, MutationBuilder};
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct MailhogAddress {
        #[serde(rename = "Mailbox")]
        mailbox: String,
        #[serde(rename = "Domain")]
        domain: String,
    }

    #[derive(Deserialize, Debug)]
    struct MailhogHeaders {
        #[serde(rename = "Subject")]
        subject: Vec<String>,
        #[serde(rename = "To")]
        to: Vec<String>,
    }

    #[derive(Deserialize, Debug)]
    struct MailhogContent {
        #[serde(rename = "Headers")]
        headers: MailhogHeaders,
    }

    #[derive(Deserialize, Debug)]
    struct MailhogMessage {
        #[serde(rename = "To")]
        to: Vec<MailhogAddress>,
        #[serde(rename = "Content")]
        content: MailhogContent,
    }

    #[derive(Deserialize, Debug)]
    struct MailhogMessages {
        total: u32,
        items: Vec<MailhogMessage>,
    }

    let client = reqwest::Client::new();

    client
        .delete("http://mailhog:8025/api/v1/messages")
        .send()
        .await
        .expect("Failed to clear MailHog messages")
        .error_for_status()
        .expect("MailHog DELETE returned error status");

    let operation = NotifyMutation::build(NotifyMutationVars {
        recipient: "test@example.com".to_string(),
        notice: NoticeInput {
            subject: "Test Subject".to_string(),
            message: "Test Message".to_string(),
        },
    });

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

    let mailhog_resp: MailhogMessages = client
        .get("http://mailhog:8025/api/v2/messages")
        .send()
        .await
        .expect("Failed to fetch MailHog messages")
        .json()
        .await
        .expect("Failed to parse MailHog response");

    println!("MailHog response: {:?}", mailhog_resp);

    assert_eq!(mailhog_resp.total, 1, "Expected 1 email to be delivered");

    let message = mailhog_resp.items.first().expect("Expected at least 1 message");
    let to_address = message.to.first().expect("Expected at least 1 recipient address");
    assert_eq!(
        format!("{}@{}", to_address.mailbox, to_address.domain),
        "test@example.com",
        "Expected recipient to be test@example.com"
    );

    let subject = &message.content.headers.subject[0];
    assert_eq!(subject, "Test Subject", "Expected subject to be 'Test Subject'");
}
