use crate::ui::background::Background;
use crate::ui::windows::WindowId;
use std::collections::HashSet;

pub mod settings;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct UiState {
    pub active_windows: HashSet<WindowId>,
    pub settings: settings::Settings,
    #[serde(skip)]
    pub background: Background,
}

impl UiState {
    pub fn update(&mut self, ctx: &egui::Context) {
        self.settings.apply(ctx);
    }

    pub fn open_window(&mut self, window: WindowId) {
        self.active_windows.insert(window);
    }

    pub fn close_window(&mut self, window: &WindowId) {
        self.active_windows.remove(window);
    }

    pub fn is_window_open(&self, window: &WindowId) -> bool {
        self.active_windows.contains(window)
    }

    pub fn toggle_window(&mut self, window: WindowId) {
        if self.is_window_open(&window) {
            self.close_window(&window);
        } else {
            self.open_window(window);
        }
    }
}
