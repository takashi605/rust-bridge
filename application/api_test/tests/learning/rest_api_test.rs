use anyhow::Result;
use reqwest;

#[tokio::test]
async fn test_simple_get_request() -> Result<()> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://example.com")
        .send()
        .await?;
    
    assert_eq!(response.status(), 200);
    
    let body = response.text().await?;
    assert!(body.contains("Example Domain"));
    
    Ok(())
}