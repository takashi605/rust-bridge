use dotenvy::dotenv;
use std::env;

use lettre::{
    message::{header, Mailbox, MultiPart, SinglePart},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

#[tokio::test]
async fn test_send_email() -> anyhow::Result<()> {
    dotenv().ok();

    // --- 1) 設定の読み込み ---
    let smtp_host = env::var("SMTP_HOST")?;
    let smtp_port: u16 = env::var("SMTP_PORT")?.parse()?;
    let mailhog_api_host = env::var("MAILHOG_API_HOST")?;
    let mailhog_api_port: u16 = env::var("MAILHOG_API_PORT")?.parse()?;
    let from_email = env::var("FROM_EMAIL")?;
    let to_email = env::var("TO_EMAIL")?;

    // --- 2) メール本文（プレーン/HTML の代替）---
    let plain_body = "こんにちは！\nRust からの最初のメールです。";
    let html_body = r#"
        <html>
          <body>
            <p>こんにちは！<br/>Rust からの <b>最初のメール</b> です。</p>
          </body>
        </html>
    "#;

    // alternative(=受信側が表示可能な方を選ぶ) を組み立て
    let body = MultiPart::alternative() // text/plain と text/html の選択肢
        .singlepart(
            SinglePart::builder()
                .header(header::ContentType::TEXT_PLAIN)
                .body(plain_body.to_string()),
        )
        .singlepart(
            SinglePart::builder()
                .header(header::ContentType::TEXT_HTML)
                .body(html_body.to_string()),
        );

    // --- 3) Message を作る ---
    let message_id = uuid::Uuid::new_v4().to_string();
    let message = Message::builder()
        .from(Mailbox::new(None, from_email.parse()?))
        .to(Mailbox::new(None, to_email.parse()?))
        .subject("【テスト】Rust から最初のメール")
        .header(header::MessageId::from(message_id.clone()))
        .multipart(body)?; // ← multipart をセット

    // --- 4) SMTP クライアントを作る ---
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&smtp_host) // ローカル接続用のため、TLSなし
        .port(smtp_port)
        .build();

    // --- 5) 送信 ---
    mailer.send(message).await?;

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://{}:{}/api/v2/messages",
            mailhog_api_host, mailhog_api_port
        ))
        .query(&[("kind", "to")])
        .query(&[("query", &to_email)])
        .send()
        .await?;

    let resp_json = resp.json::<serde_json::Value>().await?;

    let subject = email_json_utils::find_subject_by_message_id(&resp_json, &message_id)
        .expect("送信したメールが MailHog に見つかりません")
        .to_string();

    let header_line = format!("Subject : {}", subject);
    let (parsed_subject, _) =
        mailparse::parse_header(header_line.as_bytes()).expect("ヘッダのパースに失敗しました");

    assert_eq!(
        parsed_subject.get_value(),
        "【テスト】Rust から最初のメール",
        "メールの件名が期待と異なります"
    );

    Ok(())
}

mod email_json_utils {
    use serde_json::Value;

    pub fn find_subject_by_message_id<'a>(
        json: &'a serde_json::Value,
        message_id: &str,
    ) -> Option<&'a str> {
        let target_message = find_item_by_message_id(json, message_id)?;

        target_message
            .get("Content")?
            .get("Headers")?
            .get("Subject")?
            .as_array()?
            .first()?
            .as_str()
    }

    pub fn find_item_by_message_id<'a>(
        json: &'a serde_json::Value,
        message_id: &str
    ) -> Option<&'a Value> {
        json.get("items")?
            .as_array()?
            .iter()
            .find(|item| {
                // クロージャ内では ? 演算子が使えないため、and_then チェーンを使用
                item.get("Content")
                    .and_then(|content| content.get("Headers"))
                    .and_then(|headers| headers.get("Message-ID"))
                    .and_then(|msg_ids| msg_ids.as_array())
                    .and_then(|ids| ids.first())
                    .and_then(|id| id.as_str())
                    == Some(message_id)
            })
    }
}
