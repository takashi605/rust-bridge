use std::fmt::{self, Display};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageIdVO {
    id: Uuid,
}

impl MessageIdVO {
    pub fn new_uuid_v4() -> Self {
        Self { id: Uuid::new_v4() }
    }

    pub fn into_inner(self) -> Uuid {
        self.id
    }

    pub fn as_inner(&self) -> &Uuid {
        &self.id
    }
}

impl Display for MessageIdVO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_id_vo_new_uuid_generates_valid_uuid_v4() {
        let message_id = MessageIdVO::new_uuid_v4();

        assert_eq!(
            message_id.as_inner().get_version_num(),
            4,
            "Generated UUID should be version 4"
        );
    }

    #[test]
    fn test_message_id_vo_into_inner_returns_uuid() {
        let message_id = MessageIdVO::new_uuid_v4();
        let expected_id = *message_id.as_inner();

        let inner_id = message_id.into_inner();

        assert_eq!(
            inner_id, expected_id,
            "into_inner should return the internal UUID"
        );
    }

    #[test]
    fn test_message_id_vo_as_inner_returns_uuid_reference() {
        let message_id = MessageIdVO::new_uuid_v4();

        let inner_ref = message_id.as_inner();

        assert_eq!(
            inner_ref.get_version_num(),
            4,
            "as_inner should return reference to UUID v4"
        );
    }

    #[test]
    fn test_message_id_vo_different_instances_have_different_ids() {
        let id1 = MessageIdVO::new_uuid_v4();
        let id2 = MessageIdVO::new_uuid_v4();

        assert_ne!(
            id1, id2,
            "Different MessageIdVO instances should have different UUIDs"
        );
    }

    #[test]
    fn test_message_id_vo_display() {
        let message_id = MessageIdVO::new_uuid_v4();
        let display_str = format!("{}", message_id);
        let to_string_str = message_id.as_inner().to_string();

        assert_eq!(
            display_str, to_string_str,
            "Display implementation should match UUID to_string"
        );
    }
}
