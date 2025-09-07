use anyhow::Result;
use async_trait::async_trait;

use crate::domain::notifier::notice::Notice;

#[async_trait]
trait MessageSender {
    async fn send(&self, recipient: String, notice: Notice) -> Result<()>;
}

struct MessageSenderMock;
impl MessageSenderMock {
    fn new() -> Self {
        MessageSenderMock
    }
}

#[async_trait]
impl MessageSender for MessageSenderMock {
    async fn send(&self, recipient: String, notice: Notice) -> Result<()> {
        let Notice { subject, message } = notice;
        println!(
            "Mock sending notice to {}: {} - {}",
            recipient, subject, message
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::notifier::notice::Notice;

    #[tokio::test]
    async fn test_message_sender_mock() {
        let message_sender = MessageSenderMock::new();
        let recipient = "User1".to_string();
        let notice = Notice::new("Test Notice".into(), "This is a test notice.".into());
        let result = message_sender.send(recipient.clone(), notice).await;
        assert!(result.is_ok());
    }
}
