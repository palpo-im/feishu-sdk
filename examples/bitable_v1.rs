//! Example: Bitable v1 API
//!
//! ```bash
//! FEISHU_APP_ID=xxx FEISHU_APP_SECRET=xxx BITABLE_APP_TOKEN=appcnxxx cargo run --example bitable_v1
//! ```

use feishu_sdk::Client;
use feishu_sdk::core::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("FEISHU_APP_ID").unwrap_or_default();
    let app_secret = std::env::var("FEISHU_APP_SECRET").unwrap_or_default();
    let app_token = std::env::var("BITABLE_APP_TOKEN").unwrap_or_default();
    if app_id.is_empty() || app_secret.is_empty() || app_token.is_empty() {
        println!("Set FEISHU_APP_ID, FEISHU_APP_SECRET and BITABLE_APP_TOKEN to run this example.");
        return Ok(());
    }

    let client = Client::new(Config::builder(&app_id, &app_secret).build())?;
    let resp = client
        .bitable_v1_app()
        .get()
        .path_param("app_token", app_token)
        .send()
        .await?;

    println!("status={}", resp.status);
    Ok(())
}
