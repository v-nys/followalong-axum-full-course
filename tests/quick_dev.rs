use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:8095").expect("Failed to get client");
    client.do_get("/hello").await?.print().await?;
    Ok(())
}