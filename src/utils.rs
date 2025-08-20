use reqwest::Client;
use serde_json::json;
use serde_json::Value;
use std::env;
use std::sync::Arc;
use std::thread::JoinHandle;
use egui;
use tokio::runtime::Runtime;

pub async fn send_request(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key =
        env::var("GEMINI_API_KEY").map_err(|_| "GEMINI_API_KEY environment variable not set")?;

    let client = Client::new();
    let res = client
        .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent")
        .query(&[("key", &api_key)])
        .header("Content-Type", "application/json")
        .json(&json!({
    "contents": [
      {
        "parts": [
          {
            "text": prompt
          }
        ]
      }
    ]
  }))
        .send()
        .await?
        .error_for_status()?;

        let res_json: Value = res.json().await?;

    // JSON Drilling for Text or it responds with a failure notice
    let response_value = res_json
        .get("candidates")
        .and_then(|candidates| candidates.get(0))
        .and_then(|candidate| candidate.get("content"))
        .and_then(|content| content.get("parts"))
        .and_then(|parts| parts.get(0))
        .and_then(|part| part.get("text"))
        .and_then(|text| text.as_str())
        .unwrap_or("No response text found");

    Ok(response_value.to_string())
}

pub fn load_image_from_bytes(bytes: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Use image crate to decode the embedded bytes
    let img = image::load_from_memory(bytes)?.to_rgba8();

    Ok(img.into_raw())
}

pub fn create_app_icon(image_bytes: &[u8], width: u32, height: u32) -> Result<egui::IconData, Box<dyn std::error::Error>> {
    let rgba_data = load_image_from_bytes(image_bytes)?;
    
    Ok(egui::IconData {
        rgba: rgba_data,
        width,
        height,
    })
}

pub fn create_viewport_with_icon(title: &str, icon_bytes: &[u8]) -> Result<egui::ViewportBuilder, Box<dyn std::error::Error>> {
    let icon = create_app_icon(icon_bytes, 32, 32)?;
    
    Ok(egui::ViewportBuilder {
        title: Some(title.to_string()),
        icon: Some(Arc::new(icon)),
        ..egui::ViewportBuilder::default()
    })
}

pub fn spawn_async_request(prompt: String) -> JoinHandle<Result<String, ()>> {
    std::thread::spawn(move || {
        let response = Runtime::new().unwrap().block_on(async {
            send_request(prompt)
                .await
                .unwrap_or_else(|_| "Error".to_string())
        });

        Ok::<String, ()>(response.to_string())
    })
}
