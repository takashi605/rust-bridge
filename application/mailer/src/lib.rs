pub mod config;

use async_trait::async_trait;
use config::MailhogConfig;
use lettre::{
    message::{header, Mailbox, MultiPart, SinglePart},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

#[async_trait]
pub trait Mailer {
    async fn send(&self) -> anyhow::Result<String>;
}

pub struct LettreMailer;

#[async_trait]
impl Mailer for LettreMailer {
    async fn send(&self) -> anyhow::Result<String> {
        // --- 1) 設定の読み込み ---
        let config = MailhogConfig::load()?;

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
            .from(Mailbox::new(None, config.from_email.parse()?))
            .to(Mailbox::new(None, config.to_email.parse()?))
            .subject("【テスト】Rust から最初のメール")
            .header(header::MessageId::from(message_id.clone()))
            .multipart(body)?; // ← multipart をセット

        // --- 4) SMTP クライアントを作る ---
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.smtp_host) // ローカル接続用のため、TLSなし
            .port(config.smtp_port)
            .build();

        // --- 5) 送信 ---
        mailer.send(message).await?;

        Ok(message_id)
    }
}
