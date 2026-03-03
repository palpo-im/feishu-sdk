//! Example: IM v1 Message API
//!
//! ```bash
//! FEISHU_APP_ID=xxx FEISHU_APP_SECRET=xxx FEISHU_CHAT_ID=oc_xxx cargo run --example im_v1_message
//! ```

use feishu_sdk::Client;
use feishu_sdk::api::ListMessageQuery;
use feishu_sdk::core::{Config, RequestOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("FEISHU_APP_ID").unwrap_or_default();
    let app_secret = std::env::var("FEISHU_APP_SECRET").unwrap_or_default();
    let chat_id = std::env::var("FEISHU_CHAT_ID").unwrap_or_default();
    if app_id.is_empty() || app_secret.is_empty() || chat_id.is_empty() {
        println!("Set FEISHU_APP_ID, FEISHU_APP_SECRET and FEISHU_CHAT_ID to run this example.");
        return Ok(());
    }

    let client = Client::new(Config::builder(&app_id, &app_secret).build())?;
    let query = ListMessageQuery {
        container_id_type: Some("chat".to_string()),
        page_size: Some(20),
        ..Default::default()
    };
    let resp = client
        .im_v1_message()
        .list_typed(chat_id, &query, RequestOptions::default())
        .await?;

    println!("code={}, msg={}", resp.code, resp.msg);
    Ok(())
}
