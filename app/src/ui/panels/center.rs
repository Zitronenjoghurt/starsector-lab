use crate::app::{Lab, LoadStatus};
use crate::ui::icons;
use crate::ui::panels::PANEL_MARGIN;
use crate::ui::state::DatasetUi;
use crate::ui::state::data_source::DataSource;
use crate::ui::state::settings::LayoutSettings;
use crate::ui::state::view::{CenterTab, Dataset};
use crate::ui::theme::Theme;
use crate::ui::widgets::compare::Compare;
use crate::ui::widgets::sprite::SpriteCache;
use crate::ui::widgets::table::{Table, TableEntity};
use egui::{Frame, RichText, Ui};
use strum::IntoEnumIterator;

pub struct CenterPanel<'a> {
    lab: &'a mut Lab,
}

impl<'a> CenterPanel<'a> {
    pub fn new(lab: &'a mut Lab) -> Self {
        Self { lab }
    }

    pub fn show(self, ui: &mut Ui) {
        Frame::central_panel(ui.style())
            .multiply_with_opacity(0.85)
            .inner_margin(PANEL_MARGIN)
            .show(ui, |ui| {
                let clip = ui.max_rect().intersect(ui.clip_rect());
                ui.set_clip_rect(clip);

                let lab = &mut *self.lab;
                let layout = lab.ui.settings.layout;

                let pinned = match lab.ui.view.dataset {
                    Dataset::Ships => lab.ui.ships.pinned.len(),
                    Dataset::Weapons => lab.ui.weapons.pinned.len(),
                };

                ui.horizontal(|ui| {
                    for tab in CenterTab::iter() {
                        let label = if tab == CenterTab::Compare && pinned > 0 {
                            format!("{} ({pinned})", tab.label())
                        } else {
                            tab.label().to_owned()
                        };
                        ui.selectable_value(&mut lab.ui.view.center, tab, RichText::new(label));
                    }
                });
                ui.separator();

                let has_data = match lab.ui.view.dataset {
                    Dataset::Ships => !lab.data.ships.is_empty(),
                    Dataset::Weapons => !lab.data.weapons.is_empty(),
                };
                if !has_data {
                    no_data(ui, &lab.ui.data_source, &lab.status);
                    return;
                }

                let tab = lab.ui.view.center;
                match lab.ui.view.dataset {
                    Dataset::Ships => render(
                        ui,
                        "ships_table",
                        &lab.data.ships,
                        &mut lab.ui.ships,
                        &mut lab.sprites,
                        tab,
                        layout,
                    ),
                    Dataset::Weapons => render(
                        ui,
                        "weapons_table",
                        &lab.data.weapons,
                        &mut lab.ui.weapons,
                        &mut lab.sprites,
                        tab,
                        layout,
                    ),
                }
            });
    }
}

fn no_data(ui: &mut Ui, source: &DataSource, status: &LoadStatus) {
    ui.add_space(48.0);
    ui.vertical_centered(|ui| {
        ui.label(
            RichText::new(format!("{} No data loaded", icons::WARNING))
                .heading()
                .color(Theme::AMBER),
        );
        ui.add_space(10.0);

        match status {
            LoadStatus::Failed(msg) => {
                ui.label("Starsector data failed to load:");
                ui.label(RichText::new(msg).weak());
            }
            _ if source.core_dir.is_none() => {
                ui.label("Set your Starsector folder to get started.");
            }
            _ if !source.is_valid() => {
                ui.label("The selected folder doesn't contain Starsector data.");
            }
            _ => {}
        }

        ui.add_space(10.0);
        ui.label(
            RichText::new(format!(
                "Open \"{} Set Starsector folder\" in the top bar,",
                icons::FOLDER_DASHED
            ))
            .weak(),
        );
        ui.label(
            RichText::new(
                "then pick the game install, Starsector.app, or the starsector-core folder.",
            )
            .weak(),
        );
    });
}

fn render<T: TableEntity>(
    ui: &mut Ui,
    id_salt: &str,
    data: &[T],
    ds: &mut DatasetUi,
    sprites: &mut SpriteCache,
    tab: CenterTab,
    layout: LayoutSettings,
) {
    match tab {
        CenterTab::Data => {
            Table::new(id_salt, data, &mut ds.table, &mut ds.selected, sprites)
                .row_height(layout.table_row_height)
                .show(ui);
        }
        CenterTab::Compare => {
            Compare::new(
                data,
                &mut ds.pinned,
                &ds.selected,
                &mut ds.compare_search,
                sprites,
            )
            .column_width(layout.compare_column_width)
            .show(ui);
        }
    }
}
