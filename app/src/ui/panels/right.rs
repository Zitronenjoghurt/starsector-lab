use crate::app::Lab;
use crate::ui::icons;
use crate::ui::panels::{PANEL_MARGIN, panel_frame};
use crate::ui::state::DatasetUi;
use crate::ui::state::view::Dataset;
use crate::ui::theme::Theme;
use crate::ui::widgets::sprite::{SpriteCache, show_sprite};
use crate::ui::widgets::table::TableEntity;
use egui::{Align, Color32, Label, Layout, Margin, Panel, RichText, ScrollArea, Ui};
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
            .frame(panel_frame(ui.style()).inner_margin(Margin {
                left: PANEL_MARGIN.left + 8,
                ..PANEL_MARGIN
            }))
            .show(ui, |ui| {
                let lab = &mut *self.lab;

                ui.add_space(4.0);
                ui.label(
                    RichText::new(format!("{} INSPECTOR", icons::INFO))
                        .small()
                        .strong(),
                );
                ui.separator();

                let sprite_size = lab.ui.settings.layout.inspector_sprite_size;
                match lab.ui.view.dataset {
                    Dataset::Ships => ship_inspector(
                        ui,
                        &mut lab.ui.ships,
                        &lab.data.ships,
                        &mut lab.sprites,
                        sprite_size,
                    ),
                    Dataset::Weapons => generic_inspector(
                        ui,
                        &mut lab.ui.weapons,
                        &lab.data.weapons,
                        &mut lab.sprites,
                        sprite_size,
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
    sprite_size: f32,
) {
    let Some(id) = ds.selected.clone() else {
        empty(ui);
        return;
    };
    let Some(entity) = data.iter().find(|e| e.row_id() == id) else {
        return;
    };

    ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                show_sprite(ui, sprites, entity.sprite_path(), sprite_size);
                ui.add(
                    Label::new(
                        RichText::new(entity.name())
                            .color(Theme::CYAN_BRIGHT)
                            .heading(),
                    )
                    .truncate(),
                );
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
        .column(EColumn::remainder().clip(true))
        .column(EColumn::auto().clip(true))
        .body(|mut body| {
            for col in T::columns() {
                if col.id == "name" {
                    continue;
                }
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        let label =
                            ui.add(Label::new(RichText::new(col.full_label).weak()).truncate());
                        if !col.tooltip.is_empty() {
                            label.on_hover_text(col.tooltip);
                        }
                    });
                    row.col(|ui| {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.add_space(8.0);
                            ui.monospace((col.value)(entity).to_string());
                        });
                    });
                });
            }
        });
}

fn ship_inspector(
    ui: &mut Ui,
    ds: &mut DatasetUi,
    ships: &[Ship],
    sprites: &mut SpriteCache,
    sprite_size: f32,
) {
    let Some(id) = ds.selected.clone() else {
        empty(ui);
        return;
    };
    let Some(ship) = ships.iter().find(|s| s.id == id) else {
        return;
    };

    ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                show_sprite(ui, sprites, Some(&ship.layout.sprite), sprite_size);
                ui.add(
                    Label::new(
                        RichText::new(&ship.name)
                            .color(Theme::CYAN_BRIGHT)
                            .heading(),
                    )
                    .truncate(),
                );
                let size = &ship.layout.hull_size;
                ui.label(
                    RichText::new(format!("{} {}", size_icon(size), pretty_size(size)))
                        .strong()
                        .color(size_color(size)),
                )
                .on_hover_text("Hull size class");
                ui.label(RichText::new(subtitle(ship)).weak());
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
        ship.tech_manufacturer.as_deref(),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join(" · ")
}

fn size_icon(hull_size: &str) -> &'static str {
    match hull_size {
        "FIGHTER" => icons::CELL_SIGNAL_NONE,
        "FRIGATE" => icons::CELL_SIGNAL_LOW,
        "DESTROYER" => icons::CELL_SIGNAL_MEDIUM,
        "CRUISER" => icons::CELL_SIGNAL_HIGH,
        "CAPITAL_SHIP" => icons::CELL_SIGNAL_FULL,
        _ => icons::CELL_SIGNAL_X,
    }
}

fn pretty_size(hull_size: &str) -> &str {
    match hull_size {
        "FIGHTER" => "Fighter",
        "FRIGATE" => "Frigate",
        "DESTROYER" => "Destroyer",
        "CRUISER" => "Cruiser",
        "CAPITAL_SHIP" => "Capital ship",
        other => other,
    }
}

fn size_color(hull_size: &str) -> Color32 {
    match hull_size {
        "CAPITAL_SHIP" => Theme::AMBER,
        "CRUISER" => Theme::CYAN_BRIGHT,
        "DESTROYER" => Theme::CYAN,
        _ => Theme::CYAN_DIM,
    }
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
            (
                "Fleet points",
                "fleet_points",
                ship.fleet_points.to_string(),
            ),
            (
                "Ordnance",
                "ordnance_points",
                ship.ordnance_points.to_string(),
            ),
            ("Base value", "base_value", ship.base_value.to_string()),
        ],
    );
    section(
        ui,
        "DEFENSE",
        &[
            ("Hull", "hit_points", ship.hit_points.to_string()),
            ("Armor", "armor_rating", ship.armor_rating.to_string()),
            ("Flux cap", "max_flux", ship.max_flux.to_string()),
            (
                "Dissipation",
                "flux_dissipation",
                ship.flux_dissipation.to_string(),
            ),
            ("Shield", "shield_type", ship.shield_type.clone()),
            ("Shield arc", "shield_arc", opt_deg(ship.shield_arc)),
            (
                "Shield upkeep",
                "shield_upkeep",
                opt_num(ship.shield_upkeep),
            ),
            (
                "Shield eff.",
                "shield_efficiency",
                opt_num(ship.shield_efficiency),
            ),
            ("Phase cost", "phase_cost", opt_num(ship.phase_cost)),
            ("Phase upkeep", "phase_upkeep", opt_num(ship.phase_upkeep)),
        ],
    );
    section(
        ui,
        "MOBILITY",
        &[
            ("Speed", "max_speed", num(ship.max_speed)),
            ("Acceleration", "acceleration", num(ship.acceleration)),
            ("Deceleration", "deceleration", num(ship.deceleration)),
            ("Turn rate", "max_turn_rate", num(ship.max_turn_rate)),
            (
                "Turn accel.",
                "turn_acceleration",
                num(ship.turn_acceleration),
            ),
            ("Mass", "mass", num(ship.mass)),
        ],
    );
    section(
        ui,
        "LOGISTICS",
        &[
            (
                "Fighter bays",
                "fighter_bays",
                ship.fighter_bays.to_string(),
            ),
            ("Min crew", "min_crew", ship.min_crew.to_string()),
            ("Max crew", "max_crew", ship.max_crew.to_string()),
            ("Cargo", "cargo", ship.cargo.to_string()),
            ("Fuel", "fuel", ship.fuel.to_string()),
            ("Fuel / ly", "fuel_per_ly", num(ship.fuel_per_ly)),
            ("Burn", "max_burn", ship.max_burn.to_string()),
            (
                "Supplies / mo",
                "supplies_per_month",
                num(ship.supplies_per_month),
            ),
            (
                "Supplies / recovery",
                "supplies_per_recovery",
                num(ship.supplies_per_recovery),
            ),
        ],
    );
    section(
        ui,
        "COMBAT READINESS",
        &[
            (
                "CR / day",
                "cr_percent_per_day",
                num(ship.cr_percent_per_day),
            ),
            (
                "CR to deploy",
                "cr_to_deploy",
                ship.cr_to_deploy.to_string(),
            ),
            ("Peak CR (sec)", "peak_cr_sec", ship.peak_cr_sec.to_string()),
            (
                "CR loss / sec",
                "cr_loss_per_sec",
                num(ship.cr_loss_per_sec),
            ),
        ],
    );
}

fn ship_tip(id: &str) -> &'static str {
    Ship::columns()
        .iter()
        .find(|c| c.id == id)
        .map_or("", |c| c.tooltip)
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

fn section(ui: &mut Ui, title: &str, rows: &[(&str, &str, String)]) {
    ui.add_space(4.0);
    ui.label(RichText::new(title).small().strong().color(Theme::CYAN_DIM));
    TableBuilder::new(ui)
        .id_salt(title)
        .striped(true)
        .vscroll(false)
        .column(EColumn::remainder().clip(true))
        .column(EColumn::auto().clip(true))
        .body(|mut body| {
            for (label, id, value) in rows {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        let response = ui.add(Label::new(RichText::new(*label).weak()).truncate());
                        let tip = ship_tip(id);
                        if !tip.is_empty() {
                            response.on_hover_text(tip);
                        }
                    });
                    row.col(|ui| {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.add_space(8.0);
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
