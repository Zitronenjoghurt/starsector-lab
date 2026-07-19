use crate::ui::icons;
use crate::ui::theme::Theme;
use crate::ui::widgets::sprite::{SpriteCache, show_sprite};
use crate::ui::widgets::table::{Column, TableEntity};
use egui::{Align, Color32, Layout, PopupCloseBehavior, ScrollArea, TextEdit, Ui};
use egui_extras::{Column as EColumn, TableBuilder};

const HEADER_SPRITE: f32 = 56.0;
const LABEL_WIDTH: f32 = 96.0;
const ENTITY_WIDTH: f32 = 150.0;
const HEADER_HEIGHT: f32 = 120.0;
const STAT_HEIGHT: f32 = 22.0;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tier {
    Best,
    Middle,
    Worst,
}

impl Tier {
    fn tint(self) -> Color32 {
        let base = match self {
            Self::Best => Theme::GREEN,
            Self::Middle => Theme::AMBER,
            Self::Worst => Theme::RED,
        };
        Color32::from_rgba_unmultiplied(base.r(), base.g(), base.b(), 48)
    }
}

pub struct Compare<'a, T: TableEntity> {
    data: &'a [T],
    pinned: &'a mut Vec<String>,
    selected: &'a Option<String>,
    search: &'a mut String,
    sprites: &'a mut SpriteCache,
}

impl<'a, T: TableEntity> Compare<'a, T> {
    pub fn new(
        data: &'a [T],
        pinned: &'a mut Vec<String>,
        selected: &'a Option<String>,
        search: &'a mut String,
        sprites: &'a mut SpriteCache,
    ) -> Self {
        Self {
            data,
            pinned,
            selected,
            search,
            sprites,
        }
    }

    pub fn show(mut self, ui: &mut Ui) {
        self.toolbar(ui);
        ui.separator();

        let entities: Vec<&T> = self
            .pinned
            .iter()
            .filter_map(|id| self.data.iter().find(|e| e.row_id() == id))
            .collect();

        if entities.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(24.0);
                ui.weak("Select a row and pin it to start comparing.");
            });
            return;
        }

        let sprites = &mut *self.sprites;
        let fits = LABEL_WIDTH + entities.len() as f32 * ENTITY_WIDTH <= ui.available_width();
        let to_remove = if fits {
            compare_table(ui, &entities, sprites, true)
        } else {
            ScrollArea::horizontal()
                .id_salt("compare_h")
                .auto_shrink([false, false])
                .show(ui, |ui| compare_table(ui, &entities, sprites, false))
                .inner
        };

        if let Some(id) = to_remove {
            self.pinned.retain(|p| p != &id);
        }
    }

    fn toolbar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let button = ui.button(format!("{} Add", icons::PLUS));
            egui::Popup::menu(&button)
                .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
                .width(220.0)
                .show(|ui| {
                    ui.add(
                        TextEdit::singleline(self.search)
                            .hint_text("search")
                            .desired_width(f32::INFINITY),
                    );
                    let query = self.search.trim().to_lowercase();
                    ScrollArea::vertical().max_height(260.0).show(ui, |ui| {
                        let mut any = false;
                        for entity in self.data {
                            if self.pinned.iter().any(|p| p == entity.row_id()) {
                                continue;
                            }
                            if !query.is_empty() && !entity.name().to_lowercase().contains(&query) {
                                continue;
                            }
                            if ui.selectable_label(false, entity.name()).clicked() {
                                self.pinned.push(entity.row_id().to_owned());
                                self.search.clear();
                                ui.close();
                            }
                            any = true;
                        }
                        if !any {
                            ui.weak("no matches");
                        }
                    });
                });

            let selected_id = self.selected.as_deref();
            let can_pin = selected_id.is_some_and(|id| !self.pinned.iter().any(|p| p == id));
            if ui
                .add_enabled(
                    can_pin,
                    egui::Button::new(format!("{} Pin selected", icons::PUSH_PIN)),
                )
                .on_disabled_hover_text("Select a row in the Data tab first.")
                .clicked()
                && let Some(id) = selected_id
            {
                self.pinned.push(id.to_owned());
            }

            ui.label(format!("{} pinned", self.pinned.len()));

            if !self.pinned.is_empty() && ui.button("Clear").clicked() {
                self.pinned.clear();
            }
        });
    }
}

fn compare_table<T: TableEntity>(
    ui: &mut Ui,
    entities: &[&T],
    sprites: &mut SpriteCache,
    remainder: bool,
) -> Option<String> {
    let mut to_remove: Option<String> = None;

    let mut builder = TableBuilder::new(ui)
        .id_salt("compare")
        .striped(true)
        .auto_shrink([false, false])
        .min_scrolled_height(0.0)
        .cell_layout(Layout::left_to_right(Align::Center))
        .column(EColumn::exact(LABEL_WIDTH));
    for _ in entities {
        builder = builder.column(if remainder {
            EColumn::remainder()
        } else {
            EColumn::exact(ENTITY_WIDTH)
        });
    }

    builder
        .header(HEADER_HEIGHT, |mut header| {
            header.col(|ui| {
                ui.label("");
            });
            for entity in entities {
                header.col(|ui| {
                    ui.vertical_centered(|ui| {
                        show_sprite(ui, sprites, entity.sprite_path(), HEADER_SPRITE);
                        ui.strong(entity.name());
                        if ui.small_button(format!("{} remove", icons::X)).clicked() {
                            to_remove = Some(entity.row_id().to_owned());
                        }
                    });
                });
            }
        })
        .body(|mut body| {
            for col in T::columns() {
                body.row(STAT_HEIGHT, |mut row| {
                    row.col(|ui| {
                        ui.strong(col.label);
                    });
                    let ranked = rank(col, entities);
                    for (entity, tier) in entities.iter().zip(&ranked) {
                        row.col(|ui| {
                            stat_cell(ui, (col.value)(entity).to_string(), *tier, col.numeric);
                        });
                    }
                });
            }
        });

    to_remove
}

fn rank<T>(col: &Column<T>, entities: &[&T]) -> Vec<Option<Tier>> {
    let Some(higher_better) = col.higher_better else {
        return vec![None; entities.len()];
    };

    let values: Vec<Option<f64>> = entities.iter().map(|e| (col.value)(e).as_f64()).collect();
    let present: Vec<f64> = values.iter().flatten().copied().collect();
    if present.len() < 2 {
        return vec![None; entities.len()];
    }

    let max = present.iter().cloned().fold(f64::MIN, f64::max);
    let min = present.iter().cloned().fold(f64::MAX, f64::min);
    if max == min {
        return vec![None; entities.len()];
    }

    let (best, worst) = if higher_better {
        (max, min)
    } else {
        (min, max)
    };

    values
        .into_iter()
        .map(|v| {
            v.map(|v| {
                if v == best {
                    Tier::Best
                } else if v == worst {
                    Tier::Worst
                } else {
                    Tier::Middle
                }
            })
        })
        .collect()
}

fn stat_cell(ui: &mut Ui, text: String, tier: Option<Tier>, numeric: bool) {
    if let Some(tier) = tier {
        ui.painter().rect_filled(ui.max_rect(), 0.0, tier.tint());
    }
    let layout = if numeric {
        Layout::right_to_left(Align::Center)
    } else {
        Layout::left_to_right(Align::Center)
    };
    ui.with_layout(layout, |ui| {
        if numeric {
            ui.monospace(text);
        } else {
            ui.label(text);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::widgets::table::CellValue;

    struct E {
        v: Option<f64>,
    }

    fn col(higher_better: Option<bool>) -> Column<E> {
        Column {
            id: "v",
            label: "V",
            numeric: true,
            default_visible: true,
            higher_better,
            width: 10.0,
            tooltip: "",
            value: |e: &E| e.v.map_or(CellValue::Empty, CellValue::Float),
        }
    }

    fn tiers(higher_better: Option<bool>, values: &[Option<f64>]) -> Vec<Option<Tier>> {
        let entities: Vec<E> = values.iter().map(|v| E { v: *v }).collect();
        let refs: Vec<&E> = entities.iter().collect();
        rank(&col(higher_better), &refs)
    }

    #[test]
    fn best_worst_middle_when_higher_is_better() {
        assert_eq!(
            tiers(Some(true), &[Some(3.0), Some(1.0), Some(2.0)]),
            vec![Some(Tier::Best), Some(Tier::Worst), Some(Tier::Middle)]
        );
    }

    #[test]
    fn direction_flips_when_lower_is_better() {
        assert_eq!(
            tiers(Some(false), &[Some(3.0), Some(1.0), Some(2.0)]),
            vec![Some(Tier::Worst), Some(Tier::Best), Some(Tier::Middle)]
        );
    }

    #[test]
    fn missing_values_are_left_untiered() {
        assert_eq!(
            tiers(Some(true), &[Some(3.0), None, Some(1.0)]),
            vec![Some(Tier::Best), None, Some(Tier::Worst)]
        );
    }

    #[test]
    fn nothing_to_rank_leaves_everything_neutral() {
        assert_eq!(tiers(Some(true), &[Some(5.0)]), vec![None]);
        assert_eq!(tiers(Some(true), &[Some(2.0), Some(2.0)]), vec![None, None]);
        assert_eq!(tiers(None, &[Some(1.0), Some(2.0)]), vec![None, None]);
    }
}
