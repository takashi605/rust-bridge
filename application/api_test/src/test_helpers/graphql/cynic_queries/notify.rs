#[cynic::schema("app")]
mod schema {}

#[derive(cynic::QueryFragment, Debug)]
pub struct Notice {
    pub subject: String,
    pub message: String,
}

#[derive(cynic::InputObject, Debug)]
pub struct NoticeInput {
    pub subject: String,
    pub message: String,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct SendEmailPayload {
    pub recipient: String,
    pub notice: Notice,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct NotifyMutationVars {
    pub recipient: String,
    pub notice: NoticeInput,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "NotifyMutationVars")]
pub struct NotifyMutation {
    #[arguments(recipient: $recipient, notice: $notice)]
    pub notify: SendEmailPayload,
}
