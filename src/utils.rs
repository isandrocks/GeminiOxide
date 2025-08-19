use reqwest::Client;
use serde_json::json;
use serde_json::Value;
use std::env;

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

    let response_text = res.text().await?;
    let json: Value = serde_json::from_str(&response_text)?;
    let response_value = json
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
