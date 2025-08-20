

use egui::TextureHandle;
use screenshots::Screen;
use eframe::egui;


   pub fn take_full_screenshot(ctx: &egui::Context) -> Result<TextureHandle, String> {
        let screens = Screen::all().map_err(|e| format!("Failed to get screens: {}", e))?;
        let screen = screens.first().ok_or("No screens found")?;
        let image = screen.capture().map_err(|e| format!("Failed to capture: {}", e))?;
        
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

pub fn create_app_icon(image_bytes: &[u8], width: u32, height: u32) -> Result<egui::IconData, Box<dyn std::error::Error>> {
    let rgba_data = load_image_from_bytes(image_bytes)?;
    
    Ok(egui::IconData {
        rgba: rgba_data,
        width,
        height,
    })
}
