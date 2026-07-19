use crate::app::{Lab, LoadStatus};
use crate::ui::icons;
use crate::ui::panels::panel_frame;
use crate::ui::state::view::Dataset;
use crate::ui::theme::Theme;
use crate::ui::widgets::table::TableEntity;
use egui::{Layout, Panel, RichText, Ui};
use starsector_lab::data::ship::Ship;
use starsector_lab::data::weapon::Weapon;

pub struct BottomPanel<'a> {
    lab: &'a mut Lab,
}

impl<'a> BottomPanel<'a> {
    pub fn new(lab: &'a mut Lab) -> Self {
        Self { lab }
    }

    pub fn show(self, ui: &mut Ui) {
        Panel::bottom("bottom_panel")
            .frame(panel_frame(ui.style()))
            .show(ui, |ui| {
                let state = &self.lab.ui;
                let (icon, total, noun, table, sort) = match state.view.dataset {
                    Dataset::Ships => (
                        icons::ROCKET,
                        self.lab.data.ships.len(),
                        "ships",
                        &state.ships.table,
                        state.ships.table.sort_summary(Ship::columns()),
                    ),
                    Dataset::Weapons => (
                        icons::CROSSHAIR,
                        self.lab.data.weapons.len(),
                        "weapons",
                        &state.weapons.table,
                        state.weapons.table.sort_summary(Weapon::columns()),
                    ),
                };
                let status = &self.lab.status;

                ui.horizontal(|ui| {
                    ui.label(format!("{icon} {total} {noun}"));

                    if let Some(shown) = table.shown()
                        && shown != total
                    {
                        ui.separator();
                        ui.weak(format!("{shown} shown"));
                    }

                    if let Some(sort) = sort {
                        ui.separator();
                        ui.weak(format!("sort: {sort}"));
                    }

                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        if state.data_source.core_dir.is_none() {
                            ui.label(
                                RichText::new(format!("{} no core dir set", icons::WARNING))
                                    .color(Theme::AMBER),
                            );
                        } else if !state.data_source.is_valid() {
                            ui.label(
                                RichText::new(format!("{} invalid core dir", icons::WARNING))
                                    .color(Theme::RED),
                            );
                        }

                        match status {
                            LoadStatus::Loaded { ships, weapons } => {
                                ui.label(
                                    RichText::new(format!(
                                        "{} loaded {ships} ships, {weapons} weapons",
                                        icons::CHECK_CIRCLE
                                    ))
                                    .color(Theme::GREEN),
                                );
                            }
                            LoadStatus::Failed(msg) => {
                                ui.label(
                                    RichText::new(format!("{} load failed", icons::WARNING_CIRCLE))
                                        .color(Theme::RED),
                                )
                                .on_hover_text(msg);
                            }
                            LoadStatus::Idle => {}
                        }
                    });
                });
            });
    }
}
