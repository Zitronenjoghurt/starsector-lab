use crate::app::Lab;
use crate::ui::icons;
use crate::ui::state::view::Dataset;
use crate::ui::theme::Theme;
use crate::ui::widgets::window_button::WindowButton;
use crate::ui::windows::WindowId;
use egui::{Align, Frame, Layout, Panel, Response, RichText, Ui};
use starsector_lab::locate::locate_core_dirs;
use std::path::PathBuf;
use strum::IntoEnumIterator;

pub struct TopPanel<'a> {
    lab: &'a mut Lab,
}

impl<'a> TopPanel<'a> {
    pub fn new(lab: &'a mut Lab) -> Self {
        Self { lab }
    }

    fn core_dir_picker(&mut self, ui: &mut Ui) {
        let (icon, color) = if self.lab.ui.data_source.is_valid() {
            (icons::CHECK_CIRCLE, Theme::GREEN)
        } else if self.lab.ui.data_source.core_dir.is_some() {
            (icons::WARNING_CIRCLE, Theme::RED)
        } else {
            (icons::FOLDER_DASHED, Theme::CYAN_DIM)
        };

        let label = self
            .lab
            .ui
            .data_source
            .core_dir
            .as_ref()
            .and_then(|p| p.file_name().and_then(|n| n.to_str()))
            .map(|name| name.to_owned())
            .unwrap_or_else(|| "Set core dir".to_owned());

        let button = RichText::new(format!("{icon} {label}")).color(color);
        ui.menu_button(button, |ui| {
            ui.set_min_width(260.0);

            if let Some(dir) = &self.lab.ui.data_source.core_dir {
                ui.label(RichText::new(dir.to_string_lossy()).small().weak());
                ui.separator();
            }

            let mut chosen: Option<PathBuf> = None;

            ui.label(RichText::new("Detected").small().strong());
            let detected = locate_core_dirs();
            if detected.is_empty() {
                ui.weak("  none found");
            }
            for dir in &detected {
                if ui.button(dir.to_string_lossy()).clicked() {
                    chosen = Some(dir.clone());
                }
            }

            let saved: Vec<PathBuf> = self
                .lab
                .ui
                .data_source
                .saved
                .iter()
                .filter(|d| !detected.contains(d))
                .cloned()
                .collect();
            if !saved.is_empty() {
                ui.separator();
                ui.label(RichText::new("Saved").small().strong());
                for dir in saved {
                    ui.horizontal(|ui| {
                        if ui.button(dir.to_string_lossy()).clicked() {
                            chosen = Some(dir.clone());
                        }
                        if ui.small_button(icons::TRASH).clicked() {
                            self.lab.ui.data_source.forget(&dir);
                        }
                    });
                }
            }

            ui.separator();
            if ui
                .button(format!("{} Browse…", icons::FOLDER_OPEN))
                .clicked()
                && let Some(dir) = rfd::FileDialog::new().pick_folder()
            {
                chosen = Some(dir);
            }

            if let Some(dir) = chosen {
                self.lab.ui.data_source.set(dir);
                self.lab.ui.pending_reload = true;
                ui.close();
            }
        })
        .response
        .on_hover_text("Starsector core / data directory");
    }

    fn dataset_tabs(&mut self, ui: &mut Ui) {
        for dataset in Dataset::iter() {
            ui.selectable_value(&mut self.lab.ui.view.dataset, dataset, dataset.label());
        }
    }
}

impl egui::Widget for TopPanel<'_> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        Panel::top("top_panel")
            .frame(Frame::side_top_panel(ui.style()).multiply_with_opacity(0.85))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Starsector Lab").color(Theme::CYAN_BRIGHT));
                    ui.small(format!("v{}", env!("CARGO_PKG_VERSION")));
                    ui.separator();

                    self.core_dir_picker(ui);
                    ui.separator();

                    self.dataset_tabs(ui);
                    ui.separator();

                    ui.add(WindowButton::new(WindowId::Settings, &mut self.lab.ui));

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui
                            .button(icons::GITHUB_LOGO)
                            .on_hover_text("View on GitHub")
                            .clicked()
                        {
                            ui.ctx()
                                .open_url(egui::OpenUrl::new_tab(env!("CARGO_PKG_REPOSITORY")));
                        }
                    });
                })
                .response
            })
            .response
    }
}
