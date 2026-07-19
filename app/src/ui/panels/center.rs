use crate::app::Lab;
use crate::ui::state::DatasetUi;
use crate::ui::state::view::{CenterTab, Dataset};
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
            .show(ui, |ui| {
                let lab = &mut *self.lab;

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

                let tab = lab.ui.view.center;
                match lab.ui.view.dataset {
                    Dataset::Ships => render(
                        ui,
                        "ships_table",
                        &lab.data.ships,
                        &mut lab.ui.ships,
                        &mut lab.sprites,
                        tab,
                    ),
                    Dataset::Weapons => render(
                        ui,
                        "weapons_table",
                        &lab.data.weapons,
                        &mut lab.ui.weapons,
                        &mut lab.sprites,
                        tab,
                    ),
                }
            });
    }
}

fn render<T: TableEntity>(
    ui: &mut Ui,
    id_salt: &str,
    data: &[T],
    ds: &mut DatasetUi,
    sprites: &mut SpriteCache,
    tab: CenterTab,
) {
    match tab {
        CenterTab::Data => {
            Table::new(id_salt, data, &mut ds.table, &mut ds.selected, sprites).show(ui);
        }
        CenterTab::Compare => {
            Compare::new(
                data,
                &mut ds.pinned,
                &ds.selected,
                &mut ds.compare_search,
                sprites,
            )
            .show(ui);
        }
    }
}
