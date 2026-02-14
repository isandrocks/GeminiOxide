use crate::ui_components::ChatMessage;
use base64::{engine::general_purpose, Engine as _};
use egui::ColorImage;
use image::{DynamicImage, ImageBuffer, ImageFormat, RgbaImage};
use reqwest::Client;
use serde_json::json;
use serde_json::Value;
use std::thread::JoinHandle;
use tokio::runtime::Runtime;

/// System instruction for the AI assistant
const SYSTEM_INSTRUCTION: &str = r#"You are the Research-First AI Assistant, an expert in information synthesis and verification.
Primary Directive: Synthesize reliable information with high traceability. You are allergic to unsupported claims.
Evidence Policy:
   - Tier 1 (Gold Standard): Academic papers, clinical trials, laws/statutes, official government data.
   - Tier 2 (Reliable): Industry white papers, reputable journalism, textbooks.
   - Tier 3 (Treat with Caution): Blogs, social media, opinion pieces. (Must be explicitly labeled as "Informal").
Response Strategy:
Before answering, internally evaluate the quality of available information. If the answer is unknown or the evidence is weak, admit it immediately.
Required Output Sections:
1. Executive Summary & Confidence Level
Provide a direct answer or hypothesis. Explicitly state your confidence level (High/Medium/Low) based on the quality of sources.
2. Key Findings & Source Mapping
    [Fact/Claim] → Supported by: [Citation: Author, Year, Outlet]
    Note: If a source is a secondary interpretation (e.g., a news article about a study), mention the original study if possible.
3. Nuance, Conflicts & Limitations
Detail where sources disagree. Identify what is not known. Mention if data is outdated (e.g., "Most recent data is from 2019").
4. Path to Discovery (Research Leads)
    - Search Queries: Suggested boolean strings or keywords.
    - Direct links to relevant sites or databases for further investigation in MD format.
    - Venues: Specific journals, archives, or databases (e.g., PubMed, JSTOR, The Library of Congress)."#;

fn rgba_to_png(
    rgba_data: &[u8],
    width: u32,
    height: u32,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let img: RgbaImage = ImageBuffer::from_raw(width, height, rgba_data.to_vec())
        .ok_or("Failed to create image buffer from RGBA data")?;

    let dynamic_img = DynamicImage::ImageRgba8(img);
    let mut png_data = Vec::new();

    dynamic_img.write_to(&mut std::io::Cursor::new(&mut png_data), ImageFormat::Png)?;

    Ok(png_data)
}

/// Converts ColorImage to base64-encoded PNG data for API submission
fn encode_image_to_base64(img: &ColorImage) -> Result<String, Box<dyn std::error::Error>> {
    let [width, height] = img.size;

    let rgba_bytes: Vec<u8> = img
        .pixels
        .iter()
        .flat_map(|color| [color.r(), color.g(), color.b(), color.a()])
        .collect();

    let png_bytes = rgba_to_png(&rgba_bytes, width as u32, height as u32)?;
    Ok(general_purpose::STANDARD.encode(&png_bytes))
}

/// Extracts text response from Gemini API JSON response
fn extract_response_text(res_json: &Value) -> &str {
    res_json
        .pointer("/candidates/0/content/parts/0/text")
        .and_then(|v| v.as_str())
        .unwrap_or("No response text found")
}

pub async fn send_request(
    prompt: String,
    ai_model: String,
    image_data: Option<ColorImage>,
    history: Vec<ChatMessage>,
) -> Result<String, Box<dyn std::error::Error>> {
    // API key is embedded at compile time from GEMINI_API_KEY environment variable
    // Set GEMINI_API_KEY before building: cargo build --release
    const API_KEY: &str = env!("GEMINI_API_KEY");

    let api_key = API_KEY.trim();

    // Debug check for common issues
    if api_key.is_empty() {
        return Err("API key is empty. Check your .env file and rebuild.".into());
    }
    if api_key.contains("your-actual-api-key-here") || api_key.contains("placeholder") {
        return Err("API key is still a placeholder. Set a real key in .env and rebuild.".into());
    }

    let client = Client::new();

    let mut contents = Vec::new();

    // Add history
    for msg in history {
        contents.push(json!({
            "parts": [{ "text": msg.content }],
            "role": msg.role
        }));
    }

    let mut current_parts = Vec::new();

    if let Some(img) = image_data {
        let base64_data = encode_image_to_base64(&img)?;

        let image_part = json!({
            "inline_data": {
                "mime_type": "image/png",
                "data": base64_data
            }
        });

        current_parts.push(image_part);
    }

    current_parts.push(json!({
        "text": prompt
    }));

    contents.push(json!({
        "parts": current_parts,
        "role": "user",
    }));

    let res = client
        .post(format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            ai_model, api_key
        ))
        .header("Content-Type", "application/json")
        .json(&json!({
            "system_instruction": {
              "parts": [
                {
                  "text": SYSTEM_INSTRUCTION
                }
              ]
            },
            "contents": contents,
            "tools": [
                {"googleSearch": {}},
                {"urlContext": {}}
            ]
        }))
        .send()
        .await?;

    if !res.status().is_success() {
        let status = res.status();
        let error_text = res
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read error response".to_string());
        return Err(format!("HTTP Error {}: {}", status, error_text).into());
    }

    let res_json: Value = res.json().await?;

    let response_text = extract_response_text(&res_json);
    Ok(response_text.to_string())
}

pub fn spawn_async_request(
    prompt: String,
    ai_model: String,
    image_data: Option<ColorImage>,
    history: Vec<ChatMessage>, // accepts Vec<ChatMessage>
) -> JoinHandle<Result<String, ()>> {
    std::thread::spawn(move || {
        let response = Runtime::new().unwrap().block_on(async {
            send_request(prompt, ai_model, image_data, history)
                .await
                .unwrap_or_else(|err| format!("Error: {}", err))
        });

        Ok::<String, ()>(response.to_string())
    })
}
