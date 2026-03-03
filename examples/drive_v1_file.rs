//! Example: Drive v1 File API
//!
//! ```bash
//! FEISHU_APP_ID=xxx FEISHU_APP_SECRET=xxx cargo run --example drive_v1_file
//! ```

use feishu_sdk::Client;
use feishu_sdk::api::ListFileQuery;
use feishu_sdk::core::{Config, RequestOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("FEISHU_APP_ID").unwrap_or_default();
    let app_secret = std::env::var("FEISHU_APP_SECRET").unwrap_or_default();
    if app_id.is_empty() || app_secret.is_empty() {
        println!("Set FEISHU_APP_ID and FEISHU_APP_SECRET to run this example.");
        return Ok(());
    }

    let client = Client::new(Config::builder(&app_id, &app_secret).build())?;
    let query = ListFileQuery {
        page_size: Some(20),
        ..Default::default()
    };
    let resp = client
        .drive_v1_file()
        .list_typed(&query, RequestOptions::default())
        .await?;

    println!("code={}, msg={}", resp.code, resp.msg);
    Ok(())
}
