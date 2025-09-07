mod message_sender;
mod notice;

use crate::domain::notifier::notice::Notice;
use anyhow::Result;

struct Notifier {
    recipient: String,
    notice: Notice,
}

impl Notifier {
    pub fn new(recipient: String, notice: Notice) -> Self {
        Self { recipient, notice }
    }

    pub fn notify(&self) -> Result<()> {
        // TODO ここで実際の通知ロジックを実装
        // 例えば、メール送信やログ出力など。
        let Notice { subject, message } = &self.notice;

        println!(
            "Notifying {} with subject: {} and message: {}",
            self.recipient, subject, message
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_notify_message() {
        let recipient = "User1".to_string();
        let notice = Notice::new("Test Notice".into(), "This is a test notice.".into());

        let notifier = Notifier::new(recipient.clone(), notice);

        let result = notifier.notify();
        assert!(result.is_ok());
    }
}
