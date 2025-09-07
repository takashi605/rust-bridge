pub struct Notice {
    pub subject: String,
    pub message: String,
}

impl Notice {
    pub fn new(subject: String, message: String) -> Self {
        Self { subject, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notice_fields() {
        let notice = Notice::new("Test Title".into(), "Test Body".into());
        assert_eq!(notice.subject, "Test Title");
        assert_eq!(notice.message, "Test Body");
    }
}
