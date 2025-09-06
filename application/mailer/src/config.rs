use dotenvy::dotenv;
use std::env;

pub struct MailhogConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub mailhog_api_host: String,
    pub mailhog_api_port: u16,
    pub from_email: String,
    pub to_email: String,
}

impl MailhogConfig {
    pub fn load() -> anyhow::Result<Self> {
        dotenv().ok();

        let smtp_host = env::var("SMTP_HOST")?;
        let smtp_port: u16 = env::var("SMTP_PORT")?.parse()?;
        let mailhog_api_host = env::var("MAILHOG_API_HOST")?;
        let mailhog_api_port: u16 = env::var("MAILHOG_API_PORT")?.parse()?;
        let from_email = env::var("FROM_EMAIL")?;
        let to_email = env::var("TO_EMAIL")?;

        Ok(MailhogConfig {
            smtp_host,
            smtp_port,
            mailhog_api_host,
            mailhog_api_port,
            from_email,
            to_email,
        })
    }
}