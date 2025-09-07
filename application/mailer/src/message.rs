pub mod message_id;

pub use message_id::MessageIdVO;

use crate::message::MessageIdVO as MessageId;

pub struct Message {
    pub subject: String,
    pub plain_body: String,
    pub html_body: String,
    pub from_email: String,
    pub to_email: String,
    pub message_id: MessageId,
}

impl Message {
    pub fn new(
        subject: String,
        plain_body: String,
        html_body: String,
        from_email: String,
        to_email: String,
    ) -> Self {
        Self {
            subject,
            plain_body,
            html_body,
            from_email,
            to_email,
            message_id: MessageId::new_uuid_v4(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_field_access() {
        let subject = "テストメールの件名";
        let plain_body = "プレーンテキストの本文";
        let html_body = "<p>HTMLの本文</p>";
        let from_email = "sender@example.com";
        let to_email = "receiver@example.com";

        let message = Message::new(
            subject.to_string(),
            plain_body.to_string(),
            html_body.to_string(),
            from_email.to_string(),
            to_email.to_string(),
        );

        assert_eq!(message.subject, subject);
        assert_eq!(message.plain_body, plain_body);
        assert_eq!(message.html_body, html_body);
        assert_eq!(message.from_email, from_email);
        assert_eq!(message.to_email, to_email);
        // MessageIdVOに変更されたため、内部のUUIDを確認
        assert_eq!(
            message.message_id.as_inner().get_version_num(),
            4,
            "message_id should contain UUIDv4"
        );
    }
}