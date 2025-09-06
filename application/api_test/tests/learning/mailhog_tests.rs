use mailer::{config::MailhogConfig, LettreMailer, Mailer};

#[tokio::test]
async fn test_send_email() -> anyhow::Result<()> {
    // --- 1) 設定の読み込み & メール送信 ---
    let config = MailhogConfig::load()?;
    let lettre_mailer = LettreMailer;
    let message_id = lettre_mailer.send().await?;

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://{}:{}/api/v2/messages",
            config.mailhog_api_host, config.mailhog_api_port
        ))
        .query(&[("kind", "to")])
        .query(&[("query", &config.to_email)])
        .send()
        .await?;

    let resp_json = resp.json::<serde_json::Value>().await?;

    let subject = email_json_utils::find_subject_by_message_id(&resp_json, &message_id)
        .expect("送信したメールが MailHog に見つかりません")
        .to_string();

    let header_line = format!("Subject : {}", subject);
    let (parsed_subject, _) =
        mailparse::parse_header(header_line.as_bytes()).expect("ヘッダのパースに失敗しました");

    assert_eq!(
        parsed_subject.get_value(),
        "【テスト】Rust から最初のメール",
        "メールの件名が期待と異なります"
    );

    Ok(())
}

mod email_json_utils {
    use serde_json::Value;

    pub fn find_subject_by_message_id<'a>(
        json: &'a serde_json::Value,
        message_id: &str,
    ) -> Option<&'a str> {
        let target_message = find_item_by_message_id(json, message_id)?;

        target_message
            .get("Content")?
            .get("Headers")?
            .get("Subject")?
            .as_array()?
            .first()?
            .as_str()
    }

    pub fn find_item_by_message_id<'a>(
        json: &'a serde_json::Value,
        message_id: &str,
    ) -> Option<&'a Value> {
        json.get("items")?.as_array()?.iter().find(|item| {
            // クロージャ内では ? 演算子が使えないため、and_then チェーンを使用
            item.get("Content")
                .and_then(|content| content.get("Headers"))
                .and_then(|headers| headers.get("Message-ID"))
                .and_then(|msg_ids| msg_ids.as_array())
                .and_then(|ids| ids.first())
                .and_then(|id| id.as_str())
                == Some(message_id)
        })
    }
}
