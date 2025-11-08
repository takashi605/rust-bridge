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
        // TODO: HTML変換を後で改善する
        let plain_body = notice.message.clone();
        let html_body = notice.message.clone(); // 暫定的に同じ内容

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

    // テスト用のモックMailer
    struct MockMailer {
        expected_message_id: String,
    }

    #[async_trait]
    impl Mailer for MockMailer {
        fn new(_smtp_host: String, _smtp_port: u16) -> Self {
            unimplemented!("Not used in tests")
        }

        async fn send(&self, _message: Message) -> anyhow::Result<String> {
            Ok(self.expected_message_id.clone())
        }
    }

    impl MockMailer {
        fn new_with_message_id(message_id: String) -> Self {
            Self {
                expected_message_id: message_id,
            }
        }
    }

    #[tokio::test]
    async fn test_email_sender_sends_notice_successfully() {
        let from_email = "noreply@example.com".to_string();
        let mock_mailer = MockMailer::new_with_message_id("test-message-id".to_string());
        let email_sender = EmailSender::new(mock_mailer, from_email);

        let recipient = "test@example.com".to_string();
        let notice = Notice::new("Test Subject".into(), "Test Message".into());

        // email_senderがMessageSenderを実装していることを確認
        // この行でコンパイルエラーが出るはず（MessageSenderが未実装）
        let result = email_sender.send(recipient, notice).await;
        assert!(result.is_ok());
    }
}
