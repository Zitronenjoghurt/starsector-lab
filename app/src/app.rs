use eframe::Frame;
use egui::{FontDefinitions, Ui};
use egui_notify::Toasts;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Lab {
    #[serde(default, skip)]
    pub toasts: Toasts,
}

impl Lab {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.5);
        Self::setup_fonts(&cc.egui_ctx);
        cc.storage
            .and_then(|storage| eframe::get_value::<Self>(storage, eframe::APP_KEY))
            .unwrap_or_default()
    }

    fn setup_fonts(ctx: &egui::Context) {
        let mut fonts = FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        ctx.set_fonts(fonts);
    }
}

impl eframe::App for Lab {
    fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {}
}
