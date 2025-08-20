#![windows_subsystem = "windows"]
use eframe::{egui, NativeOptions};
mod utils;
mod screen_cap;
mod ui_system;
use utils::create_viewport_with_icon;
use ui_system::UIState;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok(); // just for the API key secret

    let imported_img_bytes = include_bytes!("heart_inlineBG.png");
    let custom_viewport = create_viewport_with_icon("Gemini Interface", imported_img_bytes)?;

    let custom_options = NativeOptions {
        viewport: custom_viewport,
        ..NativeOptions::default()
    };

    eframe::run_native(
        "Gemini Interface",
        custom_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )?;
    Ok(())
}

#[derive(Default)]
struct MyApp {
    ui_state: UIState,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let should_generate_from_input = self.ui_state.render_prompt_section(ui);
            let (should_generate_from_button, _screenshot_taken) = self.ui_state.render_action_buttons(ui, ctx);
            
            if should_generate_from_input || should_generate_from_button {
                let prompt_clone = self.ui_state.prompt.clone();
                self.ui_state.start_async_request(prompt_clone);
            }

            self.ui_state.render_response_section(ui);
            self.ui_state.render_loading_indicator(ui, ctx);
        });
    }
}
