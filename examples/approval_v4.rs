//! Example: Approval v4 API
//!
//! ```bash
//! FEISHU_APP_ID=xxx FEISHU_APP_SECRET=xxx APPROVAL_INSTANCE_ID=xxx cargo run --example approval_v4
//! ```

use feishu_sdk::Client;
use feishu_sdk::api::GetInstanceQuery;
use feishu_sdk::core::{Config, RequestOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("FEISHU_APP_ID").unwrap_or_default();
    let app_secret = std::env::var("FEISHU_APP_SECRET").unwrap_or_default();
    let instance_id = std::env::var("APPROVAL_INSTANCE_ID").unwrap_or_default();
    if app_id.is_empty() || app_secret.is_empty() || instance_id.is_empty() {
        println!(
            "Set FEISHU_APP_ID, FEISHU_APP_SECRET and APPROVAL_INSTANCE_ID to run this example."
        );
        return Ok(());
    }

    let client = Client::new(Config::builder(&app_id, &app_secret).build())?;
    let resp = client
        .approval_v4_instance()
        .get_typed(
            instance_id,
            &GetInstanceQuery::default(),
            RequestOptions::default(),
        )
        .await?;

    println!("code={}, msg={}", resp.code, resp.msg);
    Ok(())
}
