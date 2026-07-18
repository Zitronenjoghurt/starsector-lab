use crate::app::Lab;
use crate::ui::panels::top::TopPanel;
use crate::ui::windows::WindowId;
use egui::{CentralPanel, Frame, Widget};
use strum::IntoEnumIterator;

pub use egui_phosphor::regular as icons;

mod background;
mod panels;
pub mod state;
pub mod theme;
mod widgets;
mod windows;

pub fn ui(ui: &mut egui::Ui, lab: &mut Lab) {
    for window in WindowId::iter() {
        if lab.ui.is_window_open(&window) {
            window.show(ui, lab);
        }
    }

    TopPanel::new(lab).ui(ui);
    CentralPanel::default().frame(Frame::NONE).show(ui, |ui| {
        let rect = ui.max_rect();
        lab.ui
            .background
            .paint(ui, rect, &lab.ui.settings.background);
    });

    lab.ui.update(ui.ctx());
}
