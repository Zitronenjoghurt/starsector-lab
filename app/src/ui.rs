use crate::app::Lab;
use crate::ui::panels::bottom::BottomPanel;
use crate::ui::panels::center::CenterPanel;
use crate::ui::panels::left::LeftPanel;
use crate::ui::panels::right::RightPanel;
use crate::ui::panels::top::TopPanel;
use crate::ui::windows::WindowId;
use egui::Widget;
use strum::IntoEnumIterator;

pub use egui_phosphor::regular as icons;

mod background;
mod entities;
mod panels;
pub mod state;
pub mod theme;
pub mod widgets;
mod windows;

pub fn ui(ui: &mut egui::Ui, lab: &mut Lab) {
    let rect = ui.max_rect();
    lab.ui
        .background
        .paint(ui, rect, &lab.ui.settings.background);

    TopPanel::new(lab).ui(ui);
    BottomPanel::new(lab).show(ui);
    LeftPanel::new(lab).show(ui);
    RightPanel::new(lab).show(ui);
    CenterPanel::new(lab).show(ui);

    for window in WindowId::iter() {
        if lab.ui.is_window_open(&window) {
            window.show(ui, lab);
        }
    }

    if std::mem::take(&mut lab.ui.pending_reload) {
        lab.reload_data();
    }

    lab.ui.update(ui.ctx());
}
