use reqwest::Client;
use serde_json::json;
use serde_json::Value;
use image::ImageReader;

pub async fn send_request(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client
        .post("http://localhost:11434/api/generate")
        .json(&json!({
            "model": "llama3",
            "prompt": prompt,
            "stream": false
        }))
        .send()
        .await?;

    let response_text = res.text().await?;
    let json: Value = serde_json::from_str(&response_text)?;
    let response_value = json["response"].as_str().unwrap_or_default();

    Ok(response_value.to_string())
}

pub fn load_image_from_path(path: &std::path::Path) -> Result<Vec<u8>, image::ImageError> {
    let image = ImageReader::open(path)?.decode()?;
    let image_buffer = image.to_rgba8();
    Ok(image_buffer.into_raw())
}



