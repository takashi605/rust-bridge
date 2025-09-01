use anyhow::Result;
use reqwest;
use serde_json::Value;

#[tokio::test]
async fn test_users_endpoint() -> Result<()> {
    let client = reqwest::Client::new();
    
    // /users エンドポイントにリクエストを送信
    let response = client
        .get("http://api:8080/users")
        .send()
        .await?;
    
    // ステータスコードが200であることを確認
    assert_eq!(response.status(), 200);
    
    // JSON レスポンスを解析
    let json: Value = response.json().await?;
    
    // レスポンスが配列であることを確認
    assert!(json.is_array());
    
    // 少なくとも1人のユーザーが存在することを確認（テストデータから）
    let users = json.as_array().unwrap();
    assert!(users.len() > 0);
    
    // 最初のユーザーの構造を確認
    let first_user = &users[0];
    assert!(first_user.get("id").is_some());
    assert!(first_user.get("name").is_some());
    assert!(first_user.get("email").is_some());
    
    Ok(())
}