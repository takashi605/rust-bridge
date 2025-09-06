pub mod config;
pub mod message;

pub use message::Message;

use async_trait::async_trait;
use config::MailhogConfig;
use lettre::{
    message::{header, Mailbox, MultiPart, SinglePart},
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
};

#[async_trait]
pub trait Mailer {
    fn new(message: Message) -> Self;
    async fn send(&self) -> anyhow::Result<String>;
}


pub struct LettreMailer {
    message: Message,
}

#[async_trait]
impl Mailer for LettreMailer {
    fn new(message: Message) -> Self {
        Self { message }
    }

    async fn send(&self) -> anyhow::Result<String> {
        // --- 1) 設定の読み込み ---
        let config = MailhogConfig::load()?;

        // --- 2) メール本文（Message構造体のデータを使用）---
        let plain_body = &self.message.plain_body;
        let html_body = &self.message.html_body;

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

        // --- 3) Message を作る（Message構造体のデータを使用）---
        let message = lettre::Message::builder()
            .from(Mailbox::new(None, self.message.from_email.parse()?))
            .to(Mailbox::new(None, self.message.to_email.parse()?))
            .subject("【テスト】Rust から最初のメール")
            .header(header::MessageId::from(self.message.message_id.clone()))
            .multipart(body)?; // ← multipart をセット

        // --- 4) SMTP クライアントを作る ---
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.smtp_host) // ローカル接続用のため、TLSなし
            .port(config.smtp_port)
            .build();

        // --- 5) 送信 ---
        mailer.send(message).await?;

        Ok(self.message.message_id.clone())
    }
}
