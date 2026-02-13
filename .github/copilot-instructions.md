# GeminiOxide Copilot Instructions

## Project Context
GeminiOxide is a desktop GUI client for Google's Gemini AI, built in Rust using the `egui` framework. It prioritizes a clean interface with support for text and image inputs.

## Architectural Overview
- **UI Architecture**: Immediate Mode GUI using `eframe`/`egui`.
- **Concurrency**: The UI runs on the main thread. Network requests are offloaded to spawned threads (`std::thread::spawn`) which initialize a temporary `tokio::runtime::Runtime` to execute async `reqwest` calls.
- **State Management**: `UIState` in `src/ui_components.rs` manages all application state (prompts, responses, images, loading status).

## Critical Workflows

### 1. API Key & Build Process
**CRITICAL**: The `GEMINI_API_KEY` is injected at **compile time**.
- **Location**: `src/api_client.rs` uses `env!("GEMINI_API_KEY")`.
- **Setup**: `build.rs` reads `.env` and exports the key to `rustc-env`.
- **Implication**: Changing the API key in `.env` requires a full rebuild (`cargo build`) to take effect. A restart is not sufficient.

### 2. Async/Sync Bridge
Do not use `tokio::main` on the entry point or `await` in the UI loop.
- **Pattern**:
    1. UI component calls `spawn_async_request` (returns `JoinHandle`).
    2. Main loop polls `handle.is_finished()` inside `render_loading_indicator`.
    3. If finished, result is joined and state updated.
- **Reference**: `src/ui_components.rs` (`start_async_request` -> `render_loading_indicator`).

## Key Patterns

### Image Handling
- **Flow**: `egui::ColorImage` (UI) -> `image::DynamicImage` -> PNG Bytes -> Base64 -> Gemini API.
- **Display**: Images are converted to `egui::TextureHandle` for rendering.
- **Reference**: `src/img_utils.rs` and `src/api_client.rs` (`rgba_to_png`).

### Styling & Markdown
- **Markdown**: Response text is rendered using `egui_commonmark`.
- **Fonts**: Custom fonts (Asian character support) are loaded in `src/font_setup.rs` and applied to the context at startup.

## Common Tasks

### Adding a UI Feature
1.  Add field to `UIState` struct (`src/ui_components.rs`).
2.  Initialize in `Default for UIState`.
3.  Implement rendering logic in `impl UIState`.

### Modifying API Logic
1.  Edit `send_request` in `src/api_client.rs`.
2.  Ensure request JSON matches Gemini API specs (currently v1beta).
3.  **Warning**: Do not expose async calls directly to `ui_components.rs` functions awaiting them.

## File Map
- `src/main.rs`: App entry, window setup, main update loop.
- `src/ui_components.rs`: UI widgets, state, and event handling.
- `src/api_client.rs`: Gemini API client, JSON serialization, async runtime encapsulation.
- `build.rs`: Build-time logic, environment variable validation.
