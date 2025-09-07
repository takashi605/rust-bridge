pub mod config;
pub mod message;

pub use message::{Message, MessageIdVO};

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
        // --- MultiPart ボディの生成 ---
        let body = self.generate_lettre_multipart();

        // --- Lettre メッセージの生成 ---
        let message = self.generate_lettre_message(body)?;

        // --- SMTP クライアントを作る ---
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&self.smtp_host) // ローカル接続用のため、TLSなし
            .port(self.smtp_port)
            .build();

        // --- 送信 ---
        mailer.send(message).await?;

        Ok(self.message.message_id.to_string())
    }
}

impl LettreMailer {
    /// MultiPart ボディを生成するプライベートメソッド
    /// text/plain と text/html の alternative を組み立てます
    fn generate_lettre_multipart(&self) -> MultiPart {
        let plain_body = &self.message.plain_body;
        let html_body = &self.message.html_body;

        MultiPart::alternative() // text/plain と text/html の選択肢
            .singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_PLAIN)
                    .body(plain_body.to_string()),
            )
            .singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_HTML)
                    .body(html_body.to_string()),
            )
    }

    /// Lettre メッセージを生成するプライベートメソッド
    /// Message構造体のデータを使用してlettre::Messageを構築します
    fn generate_lettre_message(&self, body: MultiPart) -> anyhow::Result<lettre::Message> {
        lettre::Message::builder()
            .from(Mailbox::new(None, self.message.from_email.parse()?))
            .to(Mailbox::new(None, self.message.to_email.parse()?))
            .subject(&self.message.subject)
            .header(header::MessageId::from(self.message.message_id.to_string()))
            .multipart(body)
            .map_err(Into::into)
    }
}
