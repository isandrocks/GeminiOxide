

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
