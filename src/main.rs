#![windows_subsystem = "windows"]
use eframe::{egui, NativeOptions};
use egui::{Spinner, ViewportBuilder};
use std::{sync::Arc, thread::JoinHandle};
use tokio::runtime::Runtime;
mod utils;
use utils::{send_request, load_image_from_path};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load an image to use as the application icon
    let imported_img = load_image_from_path(std::path::Path::new(
        "C:/Users/TestSandRocks/Documents/GitHub/ol_spell/src/heart_inlineBG.png",
    ))?;

    let heart_icon = egui::IconData {
        rgba: imported_img,
        width: 32,
        height: 32,
    };

    // Configure the application viewport with a custom icon
    let custom_viewport = ViewportBuilder {
        title: Some("Ollama Interface".to_string()),
        icon: Some(Arc::new(heart_icon)),
        ..ViewportBuilder::default()
    };

    let custom_options = NativeOptions {
        viewport: custom_viewport,
        ..NativeOptions::default()
    };

    // Run the application with the custom options
    eframe::run_native(
        "Ollama Interface",
        custom_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )?;
    Ok(())
}

// Define the application state
#[derive(Default)]
struct MyApp {
    prompt: String,
    llm_response: String,
    is_loading: bool,
    client_thread: Option<JoinHandle<Result<String, ()>>>,
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

            ui.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui| {
                    ui.text_edit_singleline(&mut self.prompt);
                },
            );
            ui.add_space(3.0);

            // Handle the "Generate" button click
            if ui
                .add_enabled(!self.is_loading, egui::Button::new("Generate"))
                .clicked()
            {
                self.is_loading = true;
                let prompt_clone = self.prompt.clone();
                self.client_thread = Some(self.start_client_thread(prompt_clone));
            }

            ui.add_space(3.0);
            ui.heading("Response:");
            ui.separator();

            // Display the response in a scrollable area
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(&self.llm_response);
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
