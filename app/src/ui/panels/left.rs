use crate::app::Lab;
use crate::ui::icons;
use crate::ui::panels::panel_frame;
use crate::ui::state::view::Dataset;
use crate::ui::widgets::table::{TableEntity, TableState};
use egui::{Panel, RichText, ScrollArea, TextEdit, Ui};
use starsector_lab::data::ship::Ship;
use starsector_lab::data::weapon::Weapon;

pub struct LeftPanel<'a> {
    lab: &'a mut Lab,
}

impl<'a> LeftPanel<'a> {
    pub fn new(lab: &'a mut Lab) -> Self {
        Self { lab }
    }

    pub fn show(self, ui: &mut Ui) {
        Panel::left("left_panel")
            .resizable(true)
            .default_size(210.0)
            .frame(panel_frame(ui.style()))
            .show(ui, |ui| match self.lab.ui.view.dataset {
                Dataset::Ships => body::<Ship>(ui, &mut self.lab.ui.ships.table),
                Dataset::Weapons => body::<Weapon>(ui, &mut self.lab.ui.weapons.table),
            });
    }
}

fn body<T: TableEntity>(ui: &mut Ui, table: &mut TableState) {
    table.ensure_initialized(T::columns());

    ui.add_space(4.0);
    ui.label(
        RichText::new(format!("{} SEARCH", icons::MAGNIFYING_GLASS))
            .small()
            .strong(),
    );
    let response = ui.add(
        TextEdit::singleline(&mut table.search)
            .hint_text("name or id")
            .desired_width(f32::INFINITY),
    );
    if response.changed() {
        table.mark_dirty();
    }

    ui.add_space(6.0);
    ui.separator();
    ui.label(
        RichText::new(format!("{} COLUMNS", icons::COLUMNS))
            .small()
            .strong(),
    );
    ui.add_space(2.0);

    ScrollArea::vertical().show(ui, |ui| {
        for col in T::columns() {
            let mut visible = table.is_visible(col.id);
            let mut response = ui.checkbox(&mut visible, col.full_label);
            if !col.tooltip.is_empty() {
                response = response.on_hover_text(col.tooltip);
            }
            if response.changed() {
                table.set_visible(T::columns(), col.id, visible);
            }
        }
    });
}
