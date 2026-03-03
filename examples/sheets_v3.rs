//! Example: Sheets v3 API
//!
//! ```bash
//! FEISHU_APP_ID=xxx FEISHU_APP_SECRET=xxx SHEET_TOKEN=shtcnxxx cargo run --example sheets_v3
//! ```

use feishu_sdk::Client;
use feishu_sdk::core::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("FEISHU_APP_ID").unwrap_or_default();
    let app_secret = std::env::var("FEISHU_APP_SECRET").unwrap_or_default();
    let token = std::env::var("SHEET_TOKEN").unwrap_or_default();
    if app_id.is_empty() || app_secret.is_empty() || token.is_empty() {
        println!("Set FEISHU_APP_ID, FEISHU_APP_SECRET and SHEET_TOKEN to run this example.");
        return Ok(());
    }

    let client = Client::new(Config::builder(&app_id, &app_secret).build())?;
    let resp = client
        .sheets_v3_spreadsheet()
        .get()
        .path_param("spreadsheet_token", token)
        .send()
        .await?;

    println!("status={}", resp.status);
    Ok(())
}
