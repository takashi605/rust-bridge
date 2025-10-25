use async_graphql::{Context, InputObject, Object, SimpleObject};

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
        // TODO: 後で本実装に置き換える（Notifierを呼び出してメール送信）
        // ダミー実装: 受け取った値をそのまま返す
        Ok(SendEmailPayload {
            recipient,
            notice: Notice {
                subject: notice.subject,
                message: notice.message,
            },
        })
    }
}
