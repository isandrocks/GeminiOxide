use arboard::Clipboard;
use eframe::egui;
use egui::TextureHandle;
use image::RgbaImage;
use screenshots::Screen;

pub fn take_full_screenshot(ctx: &egui::Context) -> Result<TextureHandle, String> {
    let screens = Screen::all().map_err(|e| format!("Failed to get screens: {}", e))?;
    let screen = screens.first().ok_or("No screens found")?;
    let image = screen
        .capture()
        .map_err(|e| format!("Failed to capture: {}", e))?;

    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.into_raw();

    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);

    let texture = ctx.load_texture("screenshot", color_image, egui::TextureOptions::default());

    Ok(texture)
}

pub fn load_image_from_bytes(bytes: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Use image crate to decode the embedded bytes
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

pub fn image_from_clipboard(ctx: &egui::Context) -> Result<TextureHandle, String> {
    let mut clipboard =
        Clipboard::new().map_err(|e| format!("Failed to create clipboard: {}", e))?;

    match clipboard.get_image() {
        Ok(img_data) => {
            let width = img_data
                .width
                .try_into()
                .map_err(|_| "Invalid image width")?;
            let height = img_data
                .height
                .try_into()
                .map_err(|_| "Invalid image height")?;

            // Convert to ImageBuffer
            let buffer: RgbaImage = RgbaImage::from_raw(width, height, img_data.bytes.into_owned())
                .ok_or("Failed to create image buffer from raw data")?;

            let size = [width as usize, height as usize];
            let pixels = buffer.into_raw();

            let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
            let texture = ctx.load_texture(
                "clipboard_image",
                color_image,
                egui::TextureOptions::default(),
            );

            return Ok(texture);
        }
        Err(_) => Err("No image data found in clipboard".to_string()),
    }
}
