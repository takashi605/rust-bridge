use anyhow::Result;
use async_trait::async_trait;

use crate::domain::notifier::message_sender::MessageSender;
use crate::domain::notifier::notice::Notice;
use mailer::{Mailer, Message};

pub struct EmailSender<M: Mailer> {
    mailer: M,
    from_email: String,
}

impl<M: Mailer> EmailSender<M> {
    pub fn new(mailer: M, from_email: String) -> Self {
        Self { mailer, from_email }
    }

    fn notice_to_message(&self, recipient: String, notice: Notice) -> Message {
        let plain_body = notice.message.clone();
        let html_body = format!("<p>{}</p>", html_escape::encode_text(&notice.message));

        Message::new(
            notice.subject,
            plain_body,
            html_body,
            self.from_email.clone(),
            recipient,
        )
    }
}

#[async_trait]
impl<M: Mailer + Send + Sync> MessageSender for EmailSender<M> {
    async fn send(&self, recipient: String, notice: Notice) -> Result<()> {
        let message = self.notice_to_message(recipient, notice);
        self.mailer.send(message).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::notifier::notice::Notice;
    use mailer::Message;

    use std::sync::{Arc, Mutex};

    // キャプチャされたメッセージのフィールド
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    struct CapturedMessage {
        subject: String,
        plain_body: String,
        html_body: String,
        from_email: String,
        to_email: String,
    }

    // テスト用のモックMailer
    #[derive(Clone)]
    struct MockMailer {
        expected_message_id: String,
        captured_message: Arc<Mutex<Option<CapturedMessage>>>,
    }

    #[async_trait]
    impl Mailer for MockMailer {
        async fn send(&self, message: Message) -> anyhow::Result<String> {
            *self.captured_message.lock()
                .expect("Failed to lock captured_message in test MockMailer") = Some(CapturedMessage {
                subject: message.subject,
                plain_body: message.plain_body,
                html_body: message.html_body,
                from_email: message.from_email,
                to_email: message.to_email,
            });
            Ok(self.expected_message_id.clone())
        }
    }

    impl MockMailer {
        fn new_with_message_id(message_id: String) -> Self {
            Self {
                expected_message_id: message_id,
                captured_message: Arc::new(Mutex::new(None)),
            }
        }

        fn get_captured_message(&self) -> Option<CapturedMessage> {
            self.captured_message.lock()
                .expect("Failed to lock captured_message in test MockMailer")
                .clone()
        }
    }

    #[tokio::test]
    async fn test_email_sender_sends_notice_successfully() {
        let from_email = "noreply@example.com".to_string();
        let mock_mailer = MockMailer::new_with_message_id("test-message-id".to_string());
        let email_sender = EmailSender::new(mock_mailer, from_email);

        let recipient = "test@example.com".to_string();
        let notice = Notice::new("Test Subject".into(), "Test Message".into());

        let result = email_sender.send(recipient, notice).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_html_conversion_escapes_special_characters() {
        let from_email = "noreply@example.com".to_string();
        let mock_mailer = MockMailer::new_with_message_id("test-message-id".to_string());
        let email_sender = EmailSender::new(mock_mailer.clone(), from_email);

        let recipient = "test@example.com".to_string();
        let notice = Notice::new(
            "Test <Subject>".into(),
            "Message with <tags> & special chars".into(),
        );

        let result = email_sender.send(recipient.clone(), notice).await;
        assert!(result.is_ok());

        // 送信されたMessageを取得
        let message = mock_mailer.get_captured_message().expect("Message should be captured");

        // HTMLボディに特殊文字がエスケープされていることを確認
        assert!(message.html_body.contains("&lt;tags&gt;"));
        assert!(message.html_body.contains("&amp;"));
        assert!(!message.html_body.contains("<tags>"), "HTML should be escaped");

        // プレーンテキストボディは変更されていないことを確認
        assert_eq!(message.plain_body, "Message with <tags> & special chars");
    }

    #[tokio::test]
    async fn test_html_conversion_generates_valid_html() {
        let from_email = "noreply@example.com".to_string();
        let mock_mailer = MockMailer::new_with_message_id("test-message-id".to_string());
        let email_sender = EmailSender::new(mock_mailer.clone(), from_email);

        let recipient = "test@example.com".to_string();
        let notice = Notice::new("Subject".into(), "Simple message".into());

        let result = email_sender.send(recipient, notice).await;
        assert!(result.is_ok());

        let message = mock_mailer.get_captured_message().expect("Message should be captured");

        // HTMLボディが基本的なHTML構造を持つことを確認
        assert!(message.html_body.contains("<p>"));
        assert!(message.html_body.contains("</p>"));
        assert!(message.html_body.contains("Simple message"));
    }

    #[tokio::test]
    async fn test_html_conversion_handles_empty_strings() {
        let from_email = "noreply@example.com".to_string();
        let mock_mailer = MockMailer::new_with_message_id("test-message-id".to_string());
        let email_sender = EmailSender::new(mock_mailer.clone(), from_email);

        let recipient = "test@example.com".to_string();
        let notice = Notice::new("".into(), "".into());

        let result = email_sender.send(recipient, notice).await;
        assert!(result.is_ok());

        let message = mock_mailer.get_captured_message().expect("Message should be captured");

        // 空文字列でも正しいHTML構造を持つことを確認
        assert_eq!(message.html_body, "<p></p>");
        assert_eq!(message.plain_body, "");
    }

    #[tokio::test]
    async fn test_html_conversion_handles_script_injection() {
        let from_email = "noreply@example.com".to_string();
        let mock_mailer = MockMailer::new_with_message_id("test-message-id".to_string());
        let email_sender = EmailSender::new(mock_mailer.clone(), from_email);

        let recipient = "test@example.com".to_string();
        let notice = Notice::new(
            "Alert".into(),
            "<script>alert('XSS')</script>".into(),
        );

        let result = email_sender.send(recipient, notice).await;
        assert!(result.is_ok());

        let message = mock_mailer.get_captured_message().expect("Message should be captured");

        // スクリプトタグが完全にエスケープされていることを確認
        assert!(!message.html_body.contains("<script>"));
        assert!(message.html_body.contains("&lt;script&gt;"));
        assert!(message.html_body.contains("&lt;/script&gt;"));
        assert!(message.html_body.contains("alert"));

        // プレーンテキストボディは変更されていないことを確認
        assert_eq!(message.plain_body, "<script>alert('XSS')</script>");
    }

    #[tokio::test]
    async fn test_html_conversion_handles_unicode_characters() {
        let from_email = "noreply@example.com".to_string();
        let mock_mailer = MockMailer::new_with_message_id("test-message-id".to_string());
        let email_sender = EmailSender::new(mock_mailer.clone(), from_email);

        let recipient = "test@example.com".to_string();
        let notice = Notice::new(
            "日本語件名".into(),
            "日本語テスト 🎉 emoji test".into(),
        );

        let result = email_sender.send(recipient, notice).await;
        assert!(result.is_ok());

        let message = mock_mailer.get_captured_message().expect("Message should be captured");

        // Unicode文字が正しく処理されることを確認
        assert!(message.html_body.contains("日本語テスト"));
        assert!(message.html_body.contains("🎉"));
        assert!(message.html_body.contains("emoji test"));
        assert!(message.html_body.contains("<p>"));
        assert!(message.html_body.contains("</p>"));

        // プレーンテキストボディは変更されていないことを確認
        assert_eq!(message.plain_body, "日本語テスト 🎉 emoji test");
    }

    #[tokio::test]
    async fn test_html_conversion_handles_newlines() {
        let from_email = "noreply@example.com".to_string();
        let mock_mailer = MockMailer::new_with_message_id("test-message-id".to_string());
        let email_sender = EmailSender::new(mock_mailer.clone(), from_email);

        let recipient = "test@example.com".to_string();
        let notice = Notice::new(
            "Multi-line".into(),
            "Line 1\nLine 2\r\nLine 3".into(),
        );

        let result = email_sender.send(recipient, notice).await;
        assert!(result.is_ok());

        let message = mock_mailer.get_captured_message().expect("Message should be captured");

        // 改行文字が保持されることを確認
        assert!(message.html_body.contains("Line 1"));
        assert!(message.html_body.contains("Line 2"));
        assert!(message.html_body.contains("Line 3"));
        assert!(message.html_body.contains("\n") || message.html_body.contains("\r\n"));

        // プレーンテキストボディは変更されていないことを確認
        assert_eq!(message.plain_body, "Line 1\nLine 2\r\nLine 3");
    }
}
