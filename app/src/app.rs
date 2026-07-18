use crate::ui;
use crate::ui::state::UiState;
use crate::ui::theme::Theme;
use eframe::{Frame, Storage};
use egui::Ui;
use egui_notify::Toasts;
use starsector_lab::data::Data;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Lab {
    pub ui: UiState,
    #[serde(default, skip)]
    pub _data: Data,
    #[serde(default, skip)]
    pub _toasts: Toasts,
}

impl Lab {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.5);
        Theme::apply(&cc.egui_ctx);
        cc.storage
            .and_then(|storage| eframe::get_value::<Self>(storage, eframe::APP_KEY))
            .unwrap_or_default()
    }
}

impl eframe::App for Lab {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        ui::ui(ui, self);
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
