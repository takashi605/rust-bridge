use dotenvy::dotenv;
use std::env;

use lettre::{
    message::{header, Mailbox, MultiPart, SinglePart},
    AsyncSmtpTransport, Message, Tokio1Executor,
    AsyncTransport,
};


// TODO 以下は正確にはテストではなく、テストデータ送信関数である。メール取得テストにして送信部分を関数化予定。
#[tokio::test]
async fn test_send_email() -> anyhow::Result<()> {
    dotenv().ok();

    // --- 1) 設定の読み込み ---
    let smtp_host = env::var("SMTP_HOST")?;
    let smtp_port: u16 = env::var("SMTP_PORT")?.parse()?;
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
    let message = Message::builder()
        .from(Mailbox::new(None, from_email.parse()?))
        .to(Mailbox::new(None, to_email.parse()?))
        .subject("【テスト】Rust から最初のメール")
        .multipart(body)?; // ← multipart をセット

    // --- 4) SMTP クライアントを作る ---
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&smtp_host) // ローカル接続用のため、TLSなし
        .port(smtp_port)
        .build();

    // --- 5) 送信 ---
    let response = mailer.send(message).await?;
    println!("Sent! server response: {:?}", response);

    Ok(())
}