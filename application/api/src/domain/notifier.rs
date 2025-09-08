mod message_sender;
mod notice;
mod mocks {
    mod message_sender_mock;
}

use crate::domain::notifier::notice::Notice;
use anyhow::Result;

struct Notifier;

impl Notifier {
    pub fn new() -> Self {
        Self
    }

    pub fn notify(&self, recipient: String, notice: Notice) -> Result<()> {
        // TODO ここで実際の通知ロジックを実装
        // 例えば、メール送信やログ出力など。
        let Notice { subject, message } = &notice;

        println!(
            "Notifying {} with subject: {} and message: {}",
            recipient, subject, message
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_notify_message() {
        let notifier = Notifier::new();

        let recipient = "User1".to_string();
        let notice = Notice::new("Test Notice".into(), "This is a test notice.".into());

        let result = notifier.notify(recipient, notice);
        assert!(result.is_ok());
    }
}
