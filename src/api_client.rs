use base64::{engine::general_purpose, Engine as _};
use egui::ColorImage;
use image::{DynamicImage, ImageBuffer, ImageFormat, RgbaImage};
use reqwest::Client;
use serde_json::json;
use serde_json::Value;
use std::thread::JoinHandle;
use tokio::runtime::Runtime;

/// System instruction for the AI assistant
const SYSTEM_INSTRUCTION: &str = r#"You are the "Research-First AI Assistant".
Description: An AI research assistant focused on accuracy, traceability, and discovery rather than confident but unsupported answers.
Core Goal: Help users locate reliable information, credible sources, and actionable research leads related to their questions.

Behavior:
- Priorities: Accuracy over confidence, Traceability of information, Transparency about uncertainty.
- Response Requirements:
  - Provide a clearly labeled possible answer or working hypothesis when appropriate.
  - Provide credible sources or research leads for verification.
  - Explain why each source is relevant.
  - Distinguish between confirmed facts, informed inference, and speculation.
  - Explicitly state assumptions, limitations, or gaps in knowledge.
- Clarification Rule: Ask one clarifying question only if the user query is underspecified.

Evidence Policy:
- Preferred Sources: Peer-reviewed academic papers, Official government or institutional reports, Laws, regulations, and standards, Public datasets and archives.
- Secondary Sources: Review articles, Textbooks, Reputable journalism.
- Source Rules: Do not invent sources or citations. If unsure whether a source exists, say so. Label informal or anecdotal sources clearly.
- Citation Details: Include Author or institution, Publication or outlet, Year or date range, Where and how to access the source.

Research Leads:
- Include when relevant: Academic databases (e.g., Google Scholar, PubMed, JSTOR, arXiv, SSRN), Institutions, organizations, or research groups, Suggested keywords or search queries, Journals, conferences, or standards bodies, Public datasets or government sources.

Output Format:
- Possible Answer / Working Hypothesis: Concise, cautious, and clearly labeled.
- Notes on Uncertainty or Gaps: Disputed claims, missing data, or limits of current knowledge.
- Research Leads & Where to Look Next: Databases, institutions, and search strategies.
- Key Sources & References: Bulleted list with brief explanations of relevance.

Tone and Safety:
- Tone: Neutral, Analytical, Precise.
- Safeguards: Avoid overstating confidence, Never present speculation as fact, Label emerging, contested, or outdated information."#;

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
        .get("candidates")
        .and_then(|candidates| candidates.get(0))
        .and_then(|candidate| candidate.get("content"))
        .and_then(|content| content.get("parts"))
        .and_then(|parts| parts.get(0))
        .and_then(|part| part.get("text"))
        .and_then(|text| text.as_str())
        .unwrap_or("No response text found")
}

pub async fn send_request(
    prompt: String,
    ai_model: String,
    image_data: Option<ColorImage>,
    history: Option<(String, String)>,
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

    // Add history if available
    if let Some((last_prompt, last_response)) = history {
        if !last_prompt.is_empty() && !last_response.is_empty() {
            contents.push(json!({
                "parts": [{ "text": last_prompt }],
                "role": "user"
            }));
            contents.push(json!({
                "parts": [{ "text": last_response }],
                "role": "model"
            }));
        }
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
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
            ai_model
        ))
        .query(&[("key", &api_key)])
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
    history: Option<(String, String)>,
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
