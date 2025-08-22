use arboard::Clipboard;
use eframe::egui;
use egui::ColorImage;
use screenshots::Screen;
use std::path::Path;

pub fn take_full_screenshot(_ctx: &egui::Context) -> Result<ColorImage, String> {
    let screens = Screen::all().map_err(|e| format!("Failed to get screens: {}", e))?;
    let screen = screens.first().ok_or("No screens found")?;
    let image = screen
        .capture()
        .map_err(|e| format!("Failed to capture: {}", e))?;

    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.into_raw();

    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);

    Ok(color_image)
}

pub fn load_image_from_bytes(bytes: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let img = image::load_from_memory(bytes)?.to_rgba8();
    Ok(img.into_raw())
}

pub fn create_app_icon(
    image_bytes: &[u8],
    width: u32,
    height: u32,
) -> Result<egui::IconData, Box<dyn std::error::Error>> {
    let rgba_data = load_image_from_bytes(image_bytes)?;

    Ok(egui::IconData {
        rgba: rgba_data,
        width,
        height,
    })
}

pub fn image_from_clipboard(ctx: &egui::Context) -> Result<ColorImage, String> {
    let mut clipboard =
        Clipboard::new().map_err(|e| format!("Failed to create clipboard: {}", e))?;

    match clipboard.get_image() {
        Ok(img_data) => {
            return process_image_data(ctx, img_data);
        }
        Err(_) => match clipboard.get_text() {
            Ok(text) => {
                let cleaned_text = text.trim().trim_matches('"');
                let path = Path::new(cleaned_text);
                if path.exists() && is_image_file(&path) {
                    return load_image_from_file(ctx, cleaned_text);
                } else {
                    if let Some(file_path) = extract_file_path_from_text(&text) {
                        let path = Path::new(&file_path);
                        if path.exists() && is_image_file(&path) {
                            return load_image_from_file(ctx, &file_path);
                        }
                    }
                    return Err(format!(
                        "Clipboard contains text '{}' but not a valid image file path",
                        text
                    ));
                }
            }
            Err(_) => {
                return Err("No image found in clipboard.\n\nTo paste an image, try one of these methods:\n1. Right-click an image in a browser and select 'Copy Image'\n2. Take a screenshot (Ctrl+Shift+S)\n3. Copy the file path manually:\n   - Right-click image file → Properties → Copy path\n   - Or hold Shift + Right-click → 'Copy as path'".to_string());
            }
        },
    }
}

fn process_image_data(
    _ctx: &egui::Context,
    img_data: arboard::ImageData,
) -> Result<ColorImage, String> {
    let width = img_data
        .width
        .try_into()
        .map_err(|_| "Invalid image width")?;
    let height = img_data
        .height
        .try_into()
        .map_err(|_| "Invalid image height")?;
    let mut pixels = img_data.bytes.into_owned();

    // Check if all pixels are transparent - had a issue with copying from firefox
    let all_transparent = pixels.chunks_exact(4).all(|chunk| chunk[3] == 0);

    if all_transparent {
        for chunk in pixels.chunks_exact_mut(4) {
            if chunk[0] != 0 || chunk[1] != 0 || chunk[2] != 0 {
                // If RGB values exist but alpha is 0, set alpha to 255
                chunk[3] = 255;
            }
        }

        // If that didn't work, try a different approach - set all non-black pixels to opaque
        let still_all_transparent = pixels.chunks_exact(4).all(|chunk| chunk[3] == 0);
        if still_all_transparent {
            for chunk in pixels.chunks_exact_mut(4) {
                if chunk[0] != 0 || chunk[1] != 0 || chunk[2] != 0 {
                    chunk[3] = 255;
                } else {
                    // For completely black pixels, try setting them to white with full alpha
                    chunk[0] = 128; // Set to gray to see if there's any data
                    chunk[1] = 128;
                    chunk[2] = 128;
                    chunk[3] = 255;
                }
            }
        }
    }

    let size = [width, height];

    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);

    Ok(color_image)
}

fn is_image_file(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        matches!(
            extension.to_string_lossy().to_lowercase().as_str(),
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "tiff" | "ico" | "tga"
        )
    } else {
        false
    }
}

fn load_image_from_file(_ctx: &egui::Context, file_path: &str) -> Result<ColorImage, String> {
    let img = image::open(file_path)
        .map_err(|e| format!("Failed to open image file: {}", e))?
        .to_rgba8();

    let size = [img.width() as usize, img.height() as usize];
    let pixels = img.into_raw();

    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);

    Ok(color_image)
}

fn extract_file_path_from_text(text: &str) -> Option<String> {
    let trimmed = text.trim();

    if trimmed.len() > 3 && trimmed.chars().nth(1) == Some(':') {
        return Some(trimmed.to_string());
    }

    if trimmed.starts_with("file://") {
        let path = trimmed.strip_prefix("file:///")?;
        return Some(path.replace('/', "\\"));
    }

    if trimmed.starts_with('"') && trimmed.ends_with('"') {
        let unquoted = &trimmed[1..trimmed.len() - 1];
        return Some(unquoted.to_string());
    }

    None
}
