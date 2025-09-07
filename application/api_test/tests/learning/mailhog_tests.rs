use mailer::{config::MailhogConfig, LettreMailer, Mailer, Message};

#[tokio::test]
async fn test_send_email() -> anyhow::Result<()> {
    // --- メールを送信 ---
    // 設定の読み込み
    let config = MailhogConfig::load()?;

    // メール用 Message を作成
    let message = Message::new(
        "【テスト】Rust から最初のメール".to_string(),
        "これはプレーンテキストのテストメールです。".to_string(),
        "<p>これは<b>HTML</b>のテストメールです。</p>".to_string(),
        config.from_email.clone(),
        config.to_email.clone(),
    );

    // LettreMailerインスタンスを作成してメール送信
    let lettre_mailer = LettreMailer::new(config.smtp_host.clone(), config.smtp_port);
    let message_id = lettre_mailer.send(message).await?;

    // --- 送信したメールを取得して正しく送れていたか検証 ---
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
        .ok_or_else(|| {
            anyhow::anyhow!(
                "送信したメールがMailHogに見つかりません。\nMessage-ID: {}\nMailHog Response: {}",
                message_id,
                serde_json::to_string_pretty(&resp_json).unwrap_or_default()
            )
        })?;

    assert_eq!(
        subject, "【テスト】Rust から最初のメール",
        "メールの件名が期待と異なります"
    );

    Ok(())
}

mod email_json_utils {
    use serde_json::Value;

    pub fn find_subject_by_message_id(
        json: &serde_json::Value,
        message_id: &str,
    ) -> Option<String> {
        let target_message = find_item_by_message_id(json, message_id)?;

        let raw_subject = target_message
            .get("Content")?
            .get("Headers")?
            .get("Subject")?
            .as_array()?
            .first()?
            .as_str()?;

        // mailparseを使用してヘッダを解析し、パースされた件名を返す
        let header_line = format!("Subject: {}", raw_subject);
        let (parsed_header, _) = mailparse::parse_header(header_line.as_bytes()).ok()?;
        Some(parsed_header.get_value())
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
