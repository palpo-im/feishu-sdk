//! Example: Docx v1 API
//!
//! ```bash
//! FEISHU_APP_ID=xxx FEISHU_APP_SECRET=xxx DOCX_DOCUMENT_ID=doxcnxxx cargo run --example docx_v1
//! ```

use feishu_sdk::Client;
use feishu_sdk::core::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("FEISHU_APP_ID").unwrap_or_default();
    let app_secret = std::env::var("FEISHU_APP_SECRET").unwrap_or_default();
    let document_id = std::env::var("DOCX_DOCUMENT_ID").unwrap_or_default();
    if app_id.is_empty() || app_secret.is_empty() || document_id.is_empty() {
        println!("Set FEISHU_APP_ID, FEISHU_APP_SECRET and DOCX_DOCUMENT_ID to run this example.");
        return Ok(());
    }

    let client = Client::new(Config::builder(&app_id, &app_secret).build())?;
    let resp = client
        .docx_v1_document()
        .get()
        .path_param("document_id", document_id)
        .send()
        .await?;

    println!("status={}", resp.status);
    Ok(())
}
