use crate::api_client::spawn_async_request;
use crate::img_utils;
use eframe::egui;
use egui::{ColorImage, Spinner, TextureHandle};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use std::sync::Arc;
use std::thread::JoinHandle;

#[derive(Clone, Debug)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

pub struct UIState {
    pub prompt: String,
    pub last_prompt: String,
    pub llm_response: String,
    pub chat_history: Vec<ChatMessage>,
    pub show_history_window: bool,
    pub is_loading: bool,
    pub client_thread: Option<JoinHandle<Result<String, ()>>>,
    pub commonmark_cache: CommonMarkCache,
    pub captured_img: Option<ColorImage>,
    pub captured_img_texture: Option<TextureHandle>,
    pub show_image_buttons: bool,
    pub error_message: Option<String>,
    pub first_frame: bool,
    pub ai_model: String,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            prompt: String::new(),
            last_prompt: String::new(),
            llm_response: String::new(),
            chat_history: Vec::new(),
            show_history_window: false,
            is_loading: false,
            client_thread: None,
            commonmark_cache: CommonMarkCache::default(),
            captured_img: None,
            captured_img_texture: None,
            show_image_buttons: false,
            error_message: None,
            first_frame: true,
            ai_model: "gemini-2.5-pro".to_string(),
        }
    }
}

impl UIState {
    pub fn update_llm_response(&mut self, response: String) {
        self.llm_response = response.clone();
        self.chat_history.push(ChatMessage {
            role: "user".to_string(),
            content: self.last_prompt.clone(),
        });
        self.chat_history.push(ChatMessage {
            role: "model".to_string(),
            content: response,
        });
    }

    pub fn start_async_request(&mut self, prompt: String) {
        if !self.is_loading && !prompt.trim().is_empty() {
            self.is_loading = true;

            // Capture last 10 messages for context
            let history_len = self.chat_history.len();
            let start_idx = if history_len > 10 {
                history_len - 10
            } else {
                0
            };
            let history = self.chat_history[start_idx..].to_vec();

            self.last_prompt = prompt.clone();
            let sent_model = self.ai_model.clone();
            self.client_thread = Some(spawn_async_request(
                prompt,
                sent_model,
                self.captured_img.clone(),
                history,
            ));
            self.prompt.clear();
            self.llm_response.clear();
        }
    }

    pub fn get_image_texture(&mut self, ctx: &egui::Context) -> Option<&TextureHandle> {
        if let Some(ref color_img) = self.captured_img {
            if self.captured_img_texture.is_none() {
                let texture = ctx.load_texture(
                    "captured_image",
                    color_img.clone(),
                    egui::TextureOptions::default(),
                );
                self.captured_img_texture = Some(texture);
            }
            self.captured_img_texture.as_ref()
        } else {
            None
        }
    }

    pub fn set_error(&mut self, error: String) {
        self.error_message = Some(error);
    }

    pub fn clear_error(&mut self) {
        self.error_message = None;
    }

    pub fn render_prompt_section(&mut self, app_ui: &mut egui::Ui) -> bool {
        app_ui.heading("Enter a prompt:");
        app_ui.add_space(3.0);

        let mut should_generate = false;

        app_ui.with_layout(
            egui::Layout::top_down_justified(egui::Align::Center),
            |ui| {
                let response = ui.text_edit_singleline(&mut self.prompt);
                if self.first_frame {
                    response.request_focus();
                    self.first_frame = false;
                }
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    should_generate = true;
                }
            },
        );
        app_ui.add_space(3.0);

        should_generate
    }

    pub fn render_action_buttons(
        &mut self,
        app_ui: &mut egui::Ui,
        ctx: &egui::Context,
    ) -> (bool, bool) {
        let mut should_generate = false;
        let mut img_context = false;

        app_ui.horizontal(|ui| {
            if ui
                .add_enabled(!self.is_loading, egui::Button::new("Generate"))
                .clicked()
            {
                should_generate = true;
                self.clear_error();
                self.show_image_buttons = false;
            }

            let button_text = if self.show_image_buttons {
                "Hide Image Options"
            } else {
                "Add Image"
            };

            if ui
                .add_enabled(!self.is_loading, egui::Button::new(button_text))
                .clicked()
            {
                self.show_image_buttons = !self.show_image_buttons;
                self.clear_error();
            }

            if ui.button("History").clicked() {
                self.show_history_window = !self.show_history_window;
            }

            if let Some(ref _color_image) = self.captured_img {
                if ui
                    .add_enabled(!self.is_loading, egui::Button::new("Clear Image"))
                    .clicked()
                {
                    self.captured_img = None;
                    self.captured_img_texture = None; // Clear texture cache
                    self.clear_error();
                }
            }

            egui::ComboBox::new("ai_model_selector", "")
                .selected_text(&self.ai_model)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.ai_model,
                        "gemini-2.5-pro".to_string(),
                        "gemini-2.5-pro",
                    );
                    ui.selectable_value(
                        &mut self.ai_model,
                        "gemini-2.5-flash".to_string(),
                        "gemini-2.5-flash",
                    );

                    ui.selectable_value(
                        &mut self.ai_model,
                        "gemini-2.5-flash-lite".to_string(),
                        "gemini-2.5-flash-lite",
                    );
                    ui.selectable_value(
                        &mut self.ai_model,
                        "gemini-3-pro-preview".to_string(),
                        "gemini-3-pro-preview",
                    );
                });
        });

        if self.show_image_buttons {
            egui::Window::new("Image Options")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui
                            .add_enabled(!self.is_loading, egui::Button::new("Pick Image"))
                            .clicked()
                        {
                            match img_utils::pick_image_file() {
                                Ok(color_image) => {
                                    self.captured_img = Some(color_image);
                                    self.captured_img_texture = None; // Clear texture cache to force recreation
                                    img_context = true;
                                    self.show_image_buttons = false;
                                    self.clear_error();
                                }
                                Err(e) => {
                                    self.set_error(format!("Screenshot failed: {}", e));
                                }
                            }
                        }

                        if ui
                            .add_enabled(!self.is_loading, egui::Button::new("Paste Image"))
                            .clicked()
                        {
                            match img_utils::image_from_clipboard(ctx) {
                                Ok(color_image) => {
                                    self.captured_img = Some(color_image);
                                    self.captured_img_texture = None; // Clear texture cache to force recreation
                                    self.show_image_buttons = false;
                                    self.clear_error();
                                }
                                Err(e) => {
                                    self.set_error(format!("Failed to paste image: {}", e));
                                }
                            }
                        }

                        if ui.button("Cancel").clicked() {
                            self.show_image_buttons = false;
                            self.clear_error();
                        }
                    });
                });
        }

        app_ui.add_space(3.0);

        (should_generate, img_context)
    }

    pub fn render_response_section(&mut self, app_ui: &mut egui::Ui, ctx: &egui::Context) {
        app_ui.heading("Response:");
        app_ui.separator();

        egui::ScrollArea::vertical().show(app_ui, |ui| {
            if !self.llm_response.is_empty() {
                ui.label(format!("Prompt: {}", self.last_prompt));
                ui.separator();

                CommonMarkViewer::new()
                    .max_image_width(Some(ui.available_width() as usize))
                    .show(ui, &mut self.commonmark_cache, &self.llm_response);
            } else {
                ui.label("No response yet...");
            }

            if let Some(texture) = self.get_image_texture(ctx) {
                ui.separator();
                ui.label("Image:");
                ui.add(
                    egui::Image::from_texture(texture)
                        .max_width(ui.available_width())
                        .maintain_aspect_ratio(true),
                );
            }

            ui.add_space(10.0);
        });
    }

    pub fn render_error_section(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("error_message").show(ctx, |ui| {
            ui.separator();
            ui.horizontal(|ui| {
                if let Some(ref error) = self.error_message {
                    ui.colored_label(egui::Color32::RED, "⚠ Error:");
                    ui.colored_label(egui::Color32::RED, error);
                } else {
                    ui.label(" ");
                }
            });
        });
    }

    pub fn render_history_window(&mut self, ctx: &egui::Context) {
        if self.show_history_window {
            let should_close = ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("history_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("Chat History")
                    .with_inner_size([400.0, 600.0]),
                |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Immediate,
                        "This egui backend doesn't support multiple viewports"
                    );

                    let mut close_requested = false;
                    if ctx.input(|i| i.viewport().close_requested()) {
                        close_requested = true;
                    }

                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.heading("Chat History");
                        if ui.button("Clear History").clicked() {
                            self.chat_history.clear();
                        }
                        ui.separator();
                        
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for msg in &self.chat_history {
                                 ui.horizontal_wrapped(|ui| {
                                    ui.label(egui::RichText::new(format!("{}:", msg.role)).strong());
                                    ui.label(&msg.content);
                                });
                                 ui.separator();
                            }
                        });
                    });
                    
                    close_requested
                },
            );
            
            if should_close {
                self.show_history_window = false;
            }
        }
    }

    pub fn render_loading_indicator(&mut self, ctx: &egui::Context) {
        if self.is_loading {
            egui::Window::new("Loading")
                .collapsible(false)
                .resizable(false)
                .title_bar(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.add(Spinner::default().size(20.0).color(egui::Color32::RED));
                        ui.label("Processing request...");
                    });
                });

            if let Some(handle) = self.client_thread.take() {
                if handle.is_finished() {
                    if let Ok(res) = handle.join() {
                        self.is_loading = false;
                        self.first_frame = true;
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

pub fn create_viewport_with_icon(
    title: &str,
    icon_bytes: &[u8],
) -> Result<egui::ViewportBuilder, Box<dyn std::error::Error>> {
    let icon = crate::img_utils::create_app_icon(icon_bytes, 32, 32)?;

    Ok(egui::ViewportBuilder::default()
        .with_title(title)
        .with_icon(Arc::new(icon))
        .with_always_on_top())
}
