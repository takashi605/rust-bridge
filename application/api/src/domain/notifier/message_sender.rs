use anyhow::Result;
use async_trait::async_trait;

use crate::domain::notifier::notice::Notice;

#[async_trait]
pub trait MessageSender {
    async fn send(&self, recipient: String, notice: Notice) -> Result<()>;
}
