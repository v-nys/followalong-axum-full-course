use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:8095").expect("Failed to get client");
    client.do_get("/hello2/vincent").await?.print().await?;

    let req_login = client.do_post("/api/login", json!({
        "username": "demo1",
        "pwd": "welcome"
    }));
    req_login.await?.print().await?;

    client.do_get("/hello2/vincent").await?.print().await?;

    let req_create = client.do_post("/api/tickets", json!({
        "title": "First ticket"
    }));
    req_create.await?.print().await?;

    let req_create = client.do_post("/api/tickets", json!({
        "title": "Second ticket"
    }));
    req_create.await?.print().await?;

    let req_list = client.do_get("/api/tickets");
    req_list.await?.print().await?;

    let req_delete = client.do_delete("/api/tickets/1");
    req_delete.await?.print().await?;

    let req_list = client.do_get("/api/tickets");
    req_list.await?.print().await?;

    Ok(())
}
