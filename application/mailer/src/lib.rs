pub mod config;
pub mod message;

pub use message::Message;

use async_trait::async_trait;
use lettre::{
    message::{header, Mailbox, MultiPart, SinglePart},
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
};

#[async_trait]
pub trait Mailer {
    fn new(message: Message, smtp_host: String, smtp_port: u16) -> Self;
    async fn send(&self) -> anyhow::Result<String>;
}


pub struct LettreMailer {
    message: Message,
    smtp_host: String,
    smtp_port: u16,
}

#[async_trait]
impl Mailer for LettreMailer {
    fn new(message: Message, smtp_host: String, smtp_port: u16) -> Self {
        Self { 
            message,
            smtp_host,
            smtp_port,
        }
    }

    async fn send(&self) -> anyhow::Result<String> {
        // --- メール本文（Message構造体のデータを使用）---
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

        // --- Message を作る（Message構造体のデータを使用）---
        let message = lettre::Message::builder()
            .from(Mailbox::new(None, self.message.from_email.parse()?))
            .to(Mailbox::new(None, self.message.to_email.parse()?))
            .subject("【テスト】Rust から最初のメール")
            .header(header::MessageId::from(self.message.message_id.clone()))
            .multipart(body)?; // ← multipart をセット

        // --- SMTP クライアントを作る ---
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&self.smtp_host) // ローカル接続用のため、TLSなし
            .port(self.smtp_port)
            .build();

        // --- 送信 ---
        mailer.send(message).await?;

        Ok(self.message.message_id.clone())
    }
}
