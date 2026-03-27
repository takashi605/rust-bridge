use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

use crate::domain::notifier::notice::Notice;

#[automock]
#[async_trait]
pub trait MessageSender {
    async fn send(&self, recipient: String, notice: Notice) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use super::*;
    use crate::domain::notifier::notice::Notice;

    #[tokio::test]
    async fn test_message_sender_mock() {
        let recipient = "User1".to_string();
        let notice = Notice::new("Test Notice".into(), "This is a test notice.".into());

        let mut mock = MockMessageSender::new();
        mock.expect_send()
            .with(eq(recipient.clone()), eq(notice.clone()))
            .times(1)
            .returning(|_, _| Ok(()));

        let result = mock.send(recipient.clone(), notice.clone()).await;
        assert!(result.is_ok());
    }
}
