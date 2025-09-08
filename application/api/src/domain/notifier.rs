mod message_sender;
mod notice;

use crate::domain::notifier::notice::Notice;
use anyhow::Result;

struct Notifier<T: message_sender::MessageSender> {
    message_sender: T,
}

impl<T: message_sender::MessageSender> Notifier<T> {
    pub fn new(message_sender: T) -> Self {
        Self { message_sender }
    }

    pub async fn notify(&self, recipient: String, notice: Notice) -> Result<()> {
        let Notice { subject, message } = &notice;

        self.message_sender.send(recipient, notice).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_notify_message() {
        let recipient = "User1".to_string();
        let notice = Notice::new("Test Notice".into(), "This is a test notice.".into());

        let mut message_sender = message_sender::MockMessageSender::new();

        // notifier.notify の内部で呼ばれる send メソッドのモック設定
        message_sender
            .expect_send()
            .with(
                mockall::predicate::eq(recipient.clone()),
                mockall::predicate::eq(notice.clone()),
            )
            .times(1)
            .returning(|_, _| Ok(()));

        let notifier = Notifier::new(message_sender);

        let result = notifier.notify(recipient, notice).await;
        assert!(result.is_ok());
    }
}
