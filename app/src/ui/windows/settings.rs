use crate::ui::background::BackgroundSettings;
use crate::ui::state::settings::{Settings, SettingsTab};
use crate::ui::widgets::reset_slider::ResetSlider;
use egui::{Grid, Response, ScrollArea, Widget};
use strum::IntoEnumIterator;

pub struct SettingsWindow<'a> {
    settings: &'a mut Settings,
}

impl<'a> SettingsWindow<'a> {
    pub fn new(settings: &'a mut Settings) -> Self {
        Self { settings }
    }

    fn tab_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for tab in SettingsTab::iter() {
                ui.selectable_value(&mut self.settings.active_tab, tab, tab.label());
            }
        });
    }

    fn general(&mut self, ui: &mut egui::Ui) {
        ui.label("UI Scale");
        let response = ResetSlider::new(&mut self.settings.ui_scale, 0.5..=4.0)
            .step_by(0.1)
            .default_value(Settings::DEFAULT_UI_SCALE)
            .ui(ui);
        if response.drag_stopped() || (response.changed() && !response.dragged()) {
            self.settings.dirty = true;
        }
        ui.end_row();
    }

    fn background(&mut self, ui: &mut egui::Ui) {
        let bg = &mut self.settings.background;
        let def = BackgroundSettings::default();

        section(ui, "STARS");

        ui.label("Count");
        ResetSlider::new(&mut bg.star_count, 0..=10000)
            .default_value(def.star_count)
            .ui(ui);
        ui.end_row();

        ui.label("Size");
        ResetSlider::new(&mut bg.star_size, 0.1..=4.0)
            .step_by(0.1)
            .default_value(def.star_size)
            .ui(ui);
        ui.end_row();

        ui.label("Min Radius");
        ResetSlider::new(&mut bg.min_radius, 0.0..=5.0)
            .step_by(0.1)
            .default_value(def.min_radius)
            .ui(ui);
        ui.end_row();

        ui.label("Max Radius");
        ResetSlider::new(&mut bg.max_radius, 0.0..=5.0)
            .step_by(0.1)
            .default_value(def.max_radius)
            .ui(ui);
        ui.end_row();

        ui.label("Min Opacity");
        ResetSlider::new(&mut bg.min_opacity, 0.0..=1.0)
            .step_by(0.05)
            .default_value(def.min_opacity)
            .ui(ui);
        ui.end_row();

        ui.label("Max Opacity");
        ResetSlider::new(&mut bg.max_opacity, 0.0..=1.0)
            .step_by(0.05)
            .default_value(def.max_opacity)
            .ui(ui);
        ui.end_row();

        ui.label("Cyan Chance");
        ResetSlider::new(&mut bg.cyan_chance, 0.0..=1.0)
            .step_by(0.05)
            .default_value(def.cyan_chance)
            .ui(ui);
        ui.end_row();

        section(ui, "MOTION");

        ui.label("Drift Speed");
        ResetSlider::new(&mut bg.drift_speed, 0.0..=5.0)
            .step_by(0.1)
            .default_value(def.drift_speed)
            .ui(ui);
        ui.end_row();

        ui.label("Drift Angle");
        ResetSlider::new(&mut bg.drift_angle, 0.0..=360.0)
            .step_by(1.0)
            .default_value(def.drift_angle)
            .ui(ui);
        ui.end_row();

        ui.label("Parallax");
        ResetSlider::new(&mut bg.parallax, 0.0..=30.0)
            .step_by(0.5)
            .default_value(def.parallax)
            .ui(ui);
        ui.end_row();

        ui.label("Twinkle Speed");
        ResetSlider::new(&mut bg.twinkle_speed, 0.0..=5.0)
            .step_by(0.1)
            .default_value(def.twinkle_speed)
            .ui(ui);
        ui.end_row();

        ui.label("Twinkle Depth");
        ResetSlider::new(&mut bg.twinkle_depth, 0.0..=1.0)
            .step_by(0.05)
            .default_value(def.twinkle_depth)
            .ui(ui);
        ui.end_row();

        section(ui, "POINTER");

        ui.label("Parallax");
        ResetSlider::new(&mut bg.mouse_parallax, 0.0..=0.5)
            .step_by(0.01)
            .default_value(def.mouse_parallax)
            .ui(ui);
        ui.end_row();

        ui.label("Response");
        ResetSlider::new(&mut bg.mouse_response, 0.01..=1.0)
            .step_by(0.01)
            .default_value(def.mouse_response)
            .ui(ui);
        ui.end_row();

        section(ui, "VIGNETTE");

        ui.label("Strength");
        ResetSlider::new(&mut bg.vignette_strength, 0.0..=1.0)
            .step_by(0.05)
            .default_value(def.vignette_strength)
            .ui(ui);
        ui.end_row();

        ui.label("Reach");
        ResetSlider::new(&mut bg.vignette_reach, 0.5..=2.0)
            .step_by(0.05)
            .default_value(def.vignette_reach)
            .ui(ui);
        ui.end_row();

        ui.label("Segments");
        ResetSlider::new(&mut bg.vignette_segments, 3..=128)
            .default_value(def.vignette_segments)
            .ui(ui);
        ui.end_row();
    }

    fn content(&mut self, ui: &mut egui::Ui) {
        Grid::new("settings_grid")
            .num_columns(2)
            .show(ui, |ui| match self.settings.active_tab {
                SettingsTab::General => self.general(ui),
                SettingsTab::Background => self.background(ui),
            });
    }
}

impl Widget for SettingsWindow<'_> {
    fn ui(mut self, ui: &mut egui::Ui) -> Response {
        self.tab_bar(ui);
        ui.separator();
        ScrollArea::vertical()
            .show(ui, |ui| {
                ui.vertical_centered(|ui| self.content(ui)).response
            })
            .inner
    }
}

fn section(ui: &mut egui::Ui, title: &str) {
    ui.strong(title);
    ui.end_row();
}
