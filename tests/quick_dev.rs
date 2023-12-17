use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let mut hc = httpc_test::new_client("http://localhost:3000");
    
    hc.as_mut().expect("no").do_get("/hello2/Mike").await?.print().await?;
    
    let binding = hc.expect("no");
    let req_login = binding.do_post(
        "/api/login",
        json!({
            "username": "Asteroidus",
            "pwd": "asterlebg"
        })
    );
    req_login.await?.print().await?;

    Ok(())
}

// Client-side : cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
// Server-side : cargo watch -q -c -w tests/ -x run