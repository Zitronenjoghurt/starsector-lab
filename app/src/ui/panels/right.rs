use crate::app::Lab;
use crate::ui::icons;
use crate::ui::panels::panel_frame;
use crate::ui::state::DatasetUi;
use crate::ui::state::view::Dataset;
use crate::ui::theme::Theme;
use crate::ui::widgets::sprite::{SpriteCache, show_sprite};
use crate::ui::widgets::table::TableEntity;
use egui::{Align, Color32, Layout, Panel, RichText, ScrollArea, Ui};
use egui_extras::{Column as EColumn, TableBuilder};
use starsector_lab::data::ship::Ship;
use std::collections::BTreeMap;

pub struct RightPanel<'a> {
    lab: &'a mut Lab,
}

impl<'a> RightPanel<'a> {
    pub fn new(lab: &'a mut Lab) -> Self {
        Self { lab }
    }

    pub fn show(self, ui: &mut Ui) {
        Panel::right("right_panel")
            .resizable(true)
            .default_size(280.0)
            .max_size(380.0)
            .frame(panel_frame(ui.style()))
            .show(ui, |ui| {
                let lab = &mut *self.lab;

                ui.add_space(4.0);
                ui.label(
                    RichText::new(format!("{} INSPECTOR", icons::INFO))
                        .small()
                        .strong(),
                );
                ui.separator();

                match lab.ui.view.dataset {
                    Dataset::Ships => {
                        ship_inspector(ui, &mut lab.ui.ships, &lab.data.ships, &mut lab.sprites)
                    }
                    Dataset::Weapons => generic_inspector(
                        ui,
                        &mut lab.ui.weapons,
                        &lab.data.weapons,
                        &mut lab.sprites,
                    ),
                }
            });
    }
}

fn empty(ui: &mut Ui) {
    ui.add_space(16.0);
    ui.vertical_centered(|ui| ui.weak("Select a row to inspect it."));
}

fn pin_button(ui: &mut Ui, ds: &mut DatasetUi, id: &str) {
    let already = ds.is_pinned(id);
    ui.vertical_centered(|ui| {
        if ui
            .add_enabled(
                !already,
                egui::Button::new(format!("{} Pin to compare", icons::PUSH_PIN)),
            )
            .on_disabled_hover_text("Already pinned")
            .clicked()
        {
            ds.pin(id);
        }
    });
}

fn generic_inspector<T: TableEntity>(
    ui: &mut Ui,
    ds: &mut DatasetUi,
    data: &[T],
    sprites: &mut SpriteCache,
) {
    let Some(id) = ds.selected.clone() else {
        empty(ui);
        return;
    };
    let Some(entity) = data.iter().find(|e| e.row_id() == id) else {
        return;
    };

    ScrollArea::vertical().show(ui, |ui| {
        ui.vertical_centered(|ui| {
            show_sprite(ui, sprites, entity.sprite_path(), 160.0);
            ui.heading(RichText::new(entity.name()).color(Theme::CYAN_BRIGHT));
        });
        ui.add_space(6.0);
        pin_button(ui, ds, &id);
        ui.add_space(6.0);
        ui.separator();
        column_table(ui, entity);
    });
}

fn column_table<T: TableEntity>(ui: &mut Ui, entity: &T) {
    TableBuilder::new(ui)
        .id_salt("inspector_cols")
        .striped(true)
        .vscroll(false)
        .column(EColumn::auto().at_least(110.0))
        .column(EColumn::remainder())
        .body(|mut body| {
            for col in T::columns() {
                if col.id == "name" {
                    continue;
                }
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label(RichText::new(col.label).weak());
                    });
                    row.col(|ui| {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.monospace((col.value)(entity).to_string());
                        });
                    });
                });
            }
        });
}

fn ship_inspector(ui: &mut Ui, ds: &mut DatasetUi, ships: &[Ship], sprites: &mut SpriteCache) {
    let Some(id) = ds.selected.clone() else {
        empty(ui);
        return;
    };
    let Some(ship) = ships.iter().find(|s| s.id == id) else {
        return;
    };

    ScrollArea::vertical().show(ui, |ui| {
        ui.vertical_centered(|ui| {
            show_sprite(ui, sprites, Some(&ship.layout.sprite), 180.0);
            ui.heading(RichText::new(&ship.name).color(Theme::CYAN_BRIGHT));
            ui.weak(subtitle(ship));
        });

        ui.add_space(6.0);
        pin_button(ui, ds, &id);

        ui.add_space(6.0);
        ui.separator();
        weapon_mounts(ui, ship);
        stats(ship, ui);
        built_ins(ui, ship);
    });
}

fn subtitle(ship: &Ship) -> String {
    [
        ship.designation.as_deref(),
        Some(ship.layout.hull_size.as_str()),
        ship.tech_manufacturer.as_deref(),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join(" · ")
}

fn weapon_mounts(ui: &mut Ui, ship: &Ship) {
    ui.label(
        RichText::new(format!("{} MOUNTS", icons::CROSSHAIR))
            .small()
            .strong(),
    );

    let slots = &ship.layout.weapon_slots;
    if slots.is_empty() {
        ui.weak("none");
        return;
    }

    let mut groups: BTreeMap<(u8, &str, &str), Vec<&_>> = BTreeMap::new();
    for slot in slots {
        let rank = match slot.size.as_str() {
            "LARGE" => 0,
            "MEDIUM" => 1,
            "SMALL" => 2,
            _ => 3,
        };
        groups
            .entry((rank, slot.size.as_str(), slot.weapon_type.as_str()))
            .or_default()
            .push(slot);
    }

    for ((_, size, weapon_type), members) in &groups {
        let detail = members
            .iter()
            .map(|s| format!("{} {:.0}° arc at {:.0}°", s.mount, s.arc, s.angle))
            .collect::<Vec<_>>()
            .join("\n");

        ui.horizontal(|ui| {
            ui.monospace(format!("{}×", members.len()));
            ui.monospace(RichText::new(*size).color(Theme::TEXT));
            ui.label(
                RichText::new(*weapon_type)
                    .color(mount_color(weapon_type))
                    .strong(),
            );
        })
        .response
        .on_hover_text(detail);
    }
}

fn mount_color(weapon_type: &str) -> Color32 {
    match weapon_type {
        "BALLISTIC" => Theme::AMBER,
        "ENERGY" => Theme::CYAN,
        "MISSILE" => Theme::GREEN,
        "SYNERGY" => Theme::CYAN_BRIGHT,
        "HYBRID" => Theme::CYAN,
        "COMPOSITE" => Theme::AMBER,
        "UNIVERSAL" => Theme::TEXT,
        _ => Theme::CYAN_DIM,
    }
}

fn stats(ship: &Ship, ui: &mut Ui) {
    section(
        ui,
        "META",
        &[
            ("Fleet points", ship.fleet_points.to_string()),
            ("Ordnance", ship.ordnance_points.to_string()),
            ("Base value", ship.base_value.to_string()),
        ],
    );
    section(
        ui,
        "DEFENSE",
        &[
            ("Hull", ship.hit_points.to_string()),
            ("Armor", ship.armor_rating.to_string()),
            ("Flux cap", ship.max_flux.to_string()),
            ("Dissipation", ship.flux_dissipation.to_string()),
            ("Shield", ship.shield_type.clone()),
            ("Shield arc", opt_deg(ship.shield_arc)),
            ("Shield upkeep", opt_num(ship.shield_upkeep)),
            ("Shield eff.", opt_num(ship.shield_efficiency)),
            ("Phase cost", opt_num(ship.phase_cost)),
            ("Phase upkeep", opt_num(ship.phase_upkeep)),
        ],
    );
    section(
        ui,
        "MOBILITY",
        &[
            ("Speed", num(ship.max_speed)),
            ("Acceleration", num(ship.acceleration)),
            ("Deceleration", num(ship.deceleration)),
            ("Turn rate", num(ship.max_turn_rate)),
            ("Turn accel.", num(ship.turn_acceleration)),
            ("Mass", num(ship.mass)),
        ],
    );
    section(
        ui,
        "LOGISTICS",
        &[
            ("Fighter bays", ship.fighter_bays.to_string()),
            ("Min crew", ship.min_crew.to_string()),
            ("Max crew", ship.max_crew.to_string()),
            ("Cargo", ship.cargo.to_string()),
            ("Fuel", ship.fuel.to_string()),
            ("Fuel / ly", num(ship.fuel_per_ly)),
            ("Burn", ship.max_burn.to_string()),
            ("Supplies / mo", num(ship.supplies_per_month)),
            ("Supplies / recovery", num(ship.supplies_per_recovery)),
        ],
    );
    section(
        ui,
        "COMBAT READINESS",
        &[
            ("CR / day", num(ship.cr_percent_per_day)),
            ("CR to deploy", ship.cr_to_deploy.to_string()),
            ("Peak CR (sec)", ship.peak_cr_sec.to_string()),
            ("CR loss / sec", num(ship.cr_loss_per_sec)),
        ],
    );
}

fn built_ins(ui: &mut Ui, ship: &Ship) {
    let layout = &ship.layout;
    let has_any = ship.system_id.is_some()
        || !layout.built_in_mods.is_empty()
        || !layout.built_in_wings.is_empty()
        || !layout.built_in_weapons.is_empty()
        || !ship.tags.is_empty();
    if !has_any {
        return;
    }

    ui.add_space(4.0);
    ui.separator();
    ui.label(
        RichText::new(format!("{} BUILT-IN", icons::WRENCH))
            .small()
            .strong(),
    );

    if let Some(system) = &ship.system_id {
        list_row(ui, "System", std::slice::from_ref(system));
    }
    list_row(ui, "Hull mods", &layout.built_in_mods);
    list_row(ui, "Wings", &layout.built_in_wings);
    if !layout.built_in_weapons.is_empty() {
        let weapons: Vec<String> = layout.built_in_weapons.values().cloned().collect();
        list_row(ui, "Weapons", &weapons);
    }
    if !ship.tags.is_empty() {
        list_row(ui, "Tags", &ship.tags);
    }
}

fn list_row(ui: &mut Ui, label: &str, values: &[String]) {
    if values.is_empty() {
        return;
    }
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(format!("{label}:")).weak());
        ui.monospace(values.join(", "));
    });
}

fn section(ui: &mut Ui, title: &str, rows: &[(&str, String)]) {
    ui.add_space(4.0);
    ui.label(RichText::new(title).small().strong().color(Theme::CYAN_DIM));
    TableBuilder::new(ui)
        .id_salt(title)
        .striped(true)
        .vscroll(false)
        .column(EColumn::auto().at_least(110.0))
        .column(EColumn::remainder())
        .body(|mut body| {
            for (label, value) in rows {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label(RichText::new(*label).weak());
                    });
                    row.col(|ui| {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.monospace(value);
                        });
                    });
                });
            }
        });
}

fn num(v: f32) -> String {
    let s = format!("{v:.2}");
    s.trim_end_matches('0').trim_end_matches('.').to_owned()
}

fn opt_num(v: Option<f32>) -> String {
    v.map_or_else(|| "-".to_owned(), num)
}

fn opt_deg(v: Option<f32>) -> String {
    v.map_or_else(|| "-".to_owned(), |v| format!("{v:.0}°"))
}
