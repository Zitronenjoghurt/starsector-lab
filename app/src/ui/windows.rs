use crate::app::Lab;
use crate::ui::icons;
use egui::Widget;
use strum::EnumIter;

mod settings;

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, EnumIter,
)]
pub enum WindowId {
    Settings,
}

impl WindowId {
    pub fn title(&self) -> &'static str {
        match self {
            WindowId::Settings => "Settings",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            WindowId::Settings => icons::GEAR,
        }
    }

    fn collapsible(&self) -> bool {
        match self {
            WindowId::Settings => true,
        }
    }

    fn content(&self, ui: &mut egui::Ui, lab: &mut Lab) {
        match self {
            Self::Settings => settings::SettingsWindow::new(&mut lab.ui.settings).ui(ui),
        };
    }

    pub fn show(&self, ui: &mut egui::Ui, lab: &mut Lab) {
        let mut is_open = true;
        egui::Window::new(self.title())
            .open(&mut is_open)
            .collapsible(self.collapsible())
            .show(ui, |ui| {
                self.content(ui, lab);
            });
        if !is_open {
            lab.ui.close_window(self)
        }
    }
}
