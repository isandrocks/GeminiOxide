#![windows_subsystem = "windows"]
use eframe::{egui, NativeOptions};
use egui::{Spinner, ViewportBuilder};
use std::{sync::Arc, thread::JoinHandle};
use tokio::runtime::Runtime;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer}; // Add this import
mod utils;
use utils::{send_request, load_image_from_bytes};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    // Embed the image directly into the binary at compile time
    let imported_img_bytes = include_bytes!("heart_inlineBG.png");
    let imported_img = load_image_from_bytes(imported_img_bytes)?;

    let heart_icon = egui::IconData {
        rgba: imported_img,
        width: 32,
        height: 32,
    };

    // Configure the application viewport with a custom icon
    let custom_viewport = ViewportBuilder {
        title: Some("Gemini Interface".to_string()),
        icon: Some(Arc::new(heart_icon)),
        ..ViewportBuilder::default()
    };

    let custom_options = NativeOptions {
        viewport: custom_viewport,
        ..NativeOptions::default()
    };

    // Run the application with the custom options
    eframe::run_native(
        "Gemini Interface",
        custom_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )?;
    Ok(())
}

// Define the application state
#[derive(Default)]
struct MyApp {
    prompt: String,
    last_prompt: String, // Store the last sent prompt
    llm_response: String,
    is_loading: bool,
    client_thread: Option<JoinHandle<Result<String, ()>>>,
    commonmark_cache: CommonMarkCache, // Add this field
}

impl MyApp {
    // Update the response from the language model
    fn update_llm_response(&mut self, response: String) {
        self.llm_response = response;
    }

    // Start a new thread to send a request to the language model
    fn start_client_thread(&self, prompt: String) -> std::thread::JoinHandle<Result<String, ()>> {
        std::thread::spawn(move || {
            let response = Runtime::new().unwrap().block_on(async {
                send_request(prompt)
                    .await
                    .unwrap_or_else(|_| "Error".to_string())
            });

            Ok::<String, ()>(response.to_string())
        })
    }
}

impl eframe::App for MyApp {
    // Update the UI and handle user interactions
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Enter a prompt:");
            ui.add_space(3.0);

            let mut should_generate = false;
            
            ui.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui| {
                    let response = ui.text_edit_singleline(&mut self.prompt);
                    // Check if Enter was pressed while the text input has focus
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        should_generate = true;
                    }
                },
            );
            ui.add_space(3.0);

            // Handle the "Generate" button click or Enter key press
            if ui
                .add_enabled(!self.is_loading, egui::Button::new("Generate"))
                .clicked() || should_generate
            {
                if !self.is_loading && !self.prompt.trim().is_empty() {
                    self.is_loading = true;
                    let prompt_clone = self.prompt.clone();
                    self.last_prompt = self.prompt.clone(); // Store the prompt before starting
                    self.prompt.clear(); // Clear the text field immediately
                    self.client_thread = Some(self.start_client_thread(prompt_clone));
                }
            }

            ui.add_space(3.0);
            ui.heading("Response:");
            ui.separator();

            // Display the response in a scrollable area with markdown support
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Show the original prompt first
                if !self.llm_response.is_empty() {
                    ui.label(format!("Prompt: {}", self.last_prompt));
                    ui.separator();
                    
                    // Render the response with markdown formatting
                    CommonMarkViewer::new("response")
                        .show(ui, &mut self.commonmark_cache, &self.llm_response);
                } else {
                    // Show placeholder text when no response yet
                    ui.label("No response yet...");
                }
            });

            // Show a loading spinner while waiting for the response
            if self.is_loading {
                ui.add(Spinner::default().size(16.0).color(egui::Color32::RED));
                
                if let Some(handle) = self.client_thread.take() {
                    if handle.is_finished() {
                        if let Ok(res) = handle.join() {
                            self.is_loading = false;
                            self.update_llm_response(res.unwrap());
                            ctx.request_repaint();
                        }
                    } else {
                        // If the thread is not finished, put the handle back
                        self.client_thread = Some(handle);
                    }
                }
            }
        });
    }
}
