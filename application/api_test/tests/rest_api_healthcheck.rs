use anyhow::Result;
use reqwest;
use serde_json::Value;

#[tokio::test]
async fn test_health_endpoint() -> Result<()> {
    let client = reqwest::Client::new();
    
    let response = client
        .get("http://api:8080/health")
        .send()
        .await?;
    
    assert_eq!(response.status(), 200);
    
    let json: Value = response.json().await?;
    assert_eq!(json["status"], "ok");
    
    Ok(())
}