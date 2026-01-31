use async_graphql::{Context, InputObject, Object, SimpleObject};
use mailer::{config::MailhogConfig, LettreMailer};

use crate::{
    adapters::email_sender::{self, EmailSender},
    domain::{self, notifier::{self, Notifier}},
};

#[derive(InputObject)]
pub struct NoticeInput {
    pub subject: String,
    pub message: String,
}

#[derive(SimpleObject)]
pub struct Notice {
    pub subject: String,
    pub message: String,
}

#[derive(SimpleObject)]
pub struct SendEmailPayload {
    pub recipient: String,
    pub notice: Notice,
}

pub struct NoticeMutation;

#[Object]
impl NoticeMutation {
    pub async fn notify(
        &self,
        _ctx: &Context<'_>,
        recipient: String,
        notice: NoticeInput,
    ) -> async_graphql::Result<SendEmailPayload> {
        let notice = notifier::notice::Notice::new(notice.subject, notice.message);

        let config = MailhogConfig::load()?;
        let mailer = LettreMailer::new(config.smtp_host, config.smtp_port);
        let email_sender = EmailSender::new(mailer, config.from_email);

        let notifier = Notifier::new(email_sender);

        notifier.notify(recipient.clone(), notice.clone()).await?;

        Ok(SendEmailPayload {
            recipient,
            notice: Notice {
                subject: notice.subject,
                message: notice.message,
            },
        })
    }
}
