use crate::ui::background::Background;
use crate::ui::state::data_source::DataSource;
use crate::ui::state::view::ViewState;
use crate::ui::widgets::table::TableState;
use crate::ui::windows::WindowId;
use std::collections::HashSet;

pub mod data_source;
pub mod settings;
pub mod view;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct DatasetUi {
    pub table: TableState,
    #[serde(skip)]
    pub selected: Option<String>,
    #[serde(skip)]
    pub pinned: Vec<String>,
    #[serde(skip)]
    pub compare_search: String,
}

impl DatasetUi {
    pub fn on_reload(&mut self) {
        self.table.mark_dirty();
        self.selected = None;
        self.pinned.clear();
    }

    pub fn pin(&mut self, id: &str) {
        if !self.pinned.iter().any(|p| p == id) {
            self.pinned.push(id.to_owned());
        }
    }

    pub fn is_pinned(&self, id: &str) -> bool {
        self.pinned.iter().any(|p| p == id)
    }
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct UiState {
    pub active_windows: HashSet<WindowId>,
    pub settings: settings::Settings,
    pub data_source: DataSource,
    pub view: ViewState,
    #[serde(default)]
    pub ships: DatasetUi,
    #[serde(default)]
    pub weapons: DatasetUi,
    #[serde(skip)]
    pub pending_reload: bool,
    #[serde(skip)]
    pub background: Background,
}

impl UiState {
    pub fn update(&mut self, ctx: &egui::Context) {
        self.settings.apply(ctx);
    }

    pub fn on_data_reloaded(&mut self) {
        self.ships.on_reload();
        self.weapons.on_reload();
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
