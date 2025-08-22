use base64::{engine::general_purpose, Engine as _};
use egui::ColorImage;
use image::{DynamicImage, ImageBuffer, ImageFormat, RgbaImage};
use reqwest::Client;
use serde_json::json;
use serde_json::Value;
use std::env;
use std::thread::JoinHandle;
use tokio::runtime::Runtime;

// Helper function to convert RGBA bytes to PNG
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

pub async fn send_request(
    prompt: String,
    image_data: Option<ColorImage>,
) -> Result<String, Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key =
        env::var("GEMINI_API_KEY").map_err(|_| "GEMINI_API_KEY environment variable not set")?;

    // Basic API key validation
    if api_key.is_empty() {
        return Err("GEMINI_API_KEY is empty".into());
    }

    if api_key.contains("your-actual-api-key-here") || api_key.contains("placeholder") {
        return Err(
            "Please set a valid GEMINI_API_KEY in your .env file (copy from .env.example)".into(),
        );
    }

    let client = Client::new();

    let mut parts = Vec::new();

    if let Some(img) = image_data {
        let [width, height] = img.size;

        let rgba_bytes: Vec<u8> = img
            .pixels
            .iter()
            .flat_map(|color| [color.r(), color.g(), color.b(), color.a()])
            .collect();

        let png_bytes = rgba_to_png(&rgba_bytes, width as u32, height as u32)?;

        let base64_data = general_purpose::STANDARD.encode(&png_bytes);

        let image_part = json!({
            "inline_data": {
                "mime_type": "image/png",
                "data": base64_data
            }
        });

        parts.push(image_part);
    }

    parts.push(json!({
        "text": prompt
    }));

    let res = client
        .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent")
        .query(&[("key", &api_key)])
        .header("Content-Type", "application/json")
        .json(&json!({
            "contents": [
                {
                    "parts": parts
                }
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

pub fn spawn_async_request(
    prompt: String,
    image_data: Option<ColorImage>,
) -> JoinHandle<Result<String, ()>> {
    std::thread::spawn(move || {
        let response = Runtime::new().unwrap().block_on(async {
            send_request(prompt, image_data)
                .await
                .unwrap_or_else(|err| format!("Error: {}", err))
        });

        Ok::<String, ()>(response.to_string())
    })
}
