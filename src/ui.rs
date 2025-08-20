use eframe::egui;
use egui::{Spinner, TextureHandle};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use std::thread::JoinHandle;
use crate::screen_cap;
use crate::utils::spawn_async_request;

pub struct UIState {
    pub prompt: String,
    pub last_prompt: String,
    pub llm_response: String,
    pub is_loading: bool,
    pub client_thread: Option<JoinHandle<Result<String, ()>>>,
    pub commonmark_cache: CommonMarkCache,
    pub screenshot_img: Option<TextureHandle>,
    pub is_captured: bool,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            prompt: String::new(),
            last_prompt: String::new(),
            llm_response: String::new(),
            is_loading: false,
            client_thread: None,
            commonmark_cache: CommonMarkCache::default(),
            screenshot_img: None,
            is_captured: false,
        }
    }
}

impl UIState {
    pub fn update_llm_response(&mut self, response: String) {
        self.llm_response = response;
    }

    pub fn start_async_request(&mut self, prompt: String) {
        if !self.is_loading && !prompt.trim().is_empty() {
            self.is_loading = true;
            self.last_prompt = prompt.clone();
            self.prompt.clear();
            self.llm_response.clear();
            self.client_thread = Some(spawn_async_request(prompt));
        }
    }

    pub fn render_prompt_section(&mut self, ui: &mut egui::Ui) -> bool {
        ui.heading("Enter a prompt:");
        ui.add_space(3.0);

        let mut should_generate = false;

        ui.with_layout(
            egui::Layout::top_down_justified(egui::Align::Center),
            |ui| {
                let response = ui.text_edit_singleline(&mut self.prompt);
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    should_generate = true;
                }
            },
        );
        ui.add_space(3.0);

        should_generate
    }

    pub fn render_action_buttons(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) -> (bool, bool) {
        let mut should_generate = false;
        let mut screenshot_taken = false;

        ui.horizontal(|ui| {
            if ui
                .add_enabled(!self.is_loading, egui::Button::new("Generate"))
                .clicked()
            {
                should_generate = true;
            }

            if ui
                .add_enabled(!self.is_loading, egui::Button::new("Screenshot"))
                .clicked()
            {
                match screen_cap::take_full_screenshot(ctx) {
                    Ok(image) => {
                        self.is_captured = true;
                        self.screenshot_img = Some(image);
                        screenshot_taken = true;
                    }
                    Err(e) => {
                        eprintln!("Screenshot failed: {}", e);
                    }
                }
            }
        });
        ui.add_space(3.0);

        (should_generate, screenshot_taken)
    }

    pub fn render_response_section(&mut self, ui: &mut egui::Ui) {
        ui.heading("Response:");
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            if !self.llm_response.is_empty() {
                ui.label(format!("Prompt: {}", self.last_prompt));
                ui.separator();

                CommonMarkViewer::new().show(
                    ui,
                    &mut self.commonmark_cache,
                    &self.llm_response,
                );
            } else {
                ui.label("No response yet...");
            }

            if let Some(ref texture) = self.screenshot_img {
                ui.separator();
                ui.label("Screenshot:");
                ui.image(texture);
            }
        });
    }

    pub fn render_loading_indicator(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
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
                    self.client_thread = Some(handle);
                }
            }
        }
    }
}
