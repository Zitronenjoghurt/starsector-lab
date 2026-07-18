use crate::app::Lab;
use crate::ui::widgets::window_button::WindowButton;
use crate::ui::windows::WindowId;
use egui::{Panel, Response, Ui};

pub struct TopPanel<'a> {
    lab: &'a mut Lab,
}

impl<'a> TopPanel<'a> {
    pub fn new(lab: &'a mut Lab) -> Self {
        Self { lab }
    }
}

impl egui::Widget for TopPanel<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        Panel::top("top_panel")
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Starsector Lab");
                    ui.small(format!("v{}", env!("CARGO_PKG_VERSION")));
                    ui.separator();
                    WindowButton::new(WindowId::Settings, &mut self.lab.ui).ui(ui);
                })
                .response
            })
            .response
    }
}
