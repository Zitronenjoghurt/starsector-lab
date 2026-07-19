use crate::ui::icons;
use crate::ui::widgets::sprite::{SpriteCache, show_sprite};
use egui::{Align, Layout, RichText, ScrollArea, Sense, Ui};
use egui_extras::{Column as EColumn, TableBuilder};
use std::cmp::Ordering;
use std::path::Path;

const ROW_HEIGHT: f32 = 26.0;
const HEADER_HEIGHT: f32 = 24.0;

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SortDir {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub enum CellValue {
    Int(i64),
    Float(f64),
    Text(String),
    Empty,
}

impl CellValue {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Int(v) => Some(*v as f64),
            Self::Float(v) => Some(*v),
            Self::Text(_) | Self::Empty => None,
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a.cmp(b),
            (Self::Float(a), Self::Float(b)) => a.total_cmp(b),
            (Self::Text(a), Self::Text(b)) => a.cmp(b),
            (Self::Empty, Self::Empty) => Ordering::Equal,
            (Self::Empty, _) => Ordering::Greater,
            (_, Self::Empty) => Ordering::Less,
            _ => self
                .as_f64()
                .partial_cmp(&other.as_f64())
                .unwrap_or(Ordering::Equal),
        }
    }
}

impl std::fmt::Display for CellValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(v) => write!(f, "{v}"),
            Self::Float(v) => {
                let s = format!("{v:.2}");
                let trimmed = s.trim_end_matches('0').trim_end_matches('.');
                write!(f, "{trimmed}")
            }
            Self::Text(v) => write!(f, "{v}"),
            Self::Empty => write!(f, "-"),
        }
    }
}

pub struct Column<T> {
    /// Stable identifier, used for persistence of visibility and sort order.
    pub id: &'static str,
    /// Header label.
    pub label: &'static str,
    /// Right-align the cell (for numbers).
    pub numeric: bool,
    /// Whether the column is shown by default (before the user customises).
    pub default_visible: bool,
    /// For the compare view: `Some(true)` if a higher value is better, `Some(false)`
    /// if lower is better, `None` to skip ranking/coloring (e.g. text columns).
    pub higher_better: Option<bool>,
    /// Initial column width in points.
    pub width: f32,
    /// Hover tooltip explaining the column (empty for none).
    pub tooltip: &'static str,
    /// Extracts the cell value from an entity.
    pub value: fn(&T) -> CellValue,
}

pub trait TableEntity: 'static {
    /// Stable unique id for selection / pinning.
    fn row_id(&self) -> &str;
    /// Human-readable name, used as the sort tiebreaker and search target.
    fn name(&self) -> &str;
    /// Absolute path to this entity's sprite, if any.
    fn sprite_path(&self) -> Option<&Path>;
    /// The column set for this entity type.
    fn columns() -> &'static [Column<Self>]
    where
        Self: Sized;
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct TableState {
    pub search: String,
    /// Visible column ids, in display order.
    visible: Vec<String>,
    /// Active sort keys, most-significant first.
    sort: Vec<(String, SortDir)>,
    /// Whether `visible`/`sort` have been seeded from the column defaults yet.
    #[serde(skip)]
    initialized: bool,
    /// Cached filtered+sorted row indices. `None` means "recompute".
    #[serde(skip)]
    order: Option<Vec<usize>>,
}

impl TableState {
    pub fn mark_dirty(&mut self) {
        self.order = None;
    }

    pub fn ensure_initialized<T: TableEntity>(&mut self, columns: &[Column<T>]) {
        if self.initialized {
            return;
        }
        self.initialized = true;
        if self.visible.is_empty() {
            self.visible = columns
                .iter()
                .filter(|c| c.default_visible)
                .map(|c| c.id.to_owned())
                .collect();
        }
    }

    pub fn is_visible(&self, id: &str) -> bool {
        self.visible.iter().any(|v| v == id)
    }

    pub fn set_visible<T: TableEntity>(&mut self, columns: &[Column<T>], id: &str, visible: bool) {
        if visible {
            if !self.is_visible(id) {
                self.visible.push(id.to_owned());
                // Re-sort visible ids into the canonical column order.
                self.visible.sort_by_key(|vid| {
                    columns
                        .iter()
                        .position(|c| c.id == vid)
                        .unwrap_or(usize::MAX)
                });
            }
        } else {
            self.visible.retain(|v| v != id);
        }
    }

    fn sort_of(&self, id: &str) -> Option<(SortDir, usize)> {
        self.sort
            .iter()
            .position(|(cid, _)| cid == id)
            .map(|pos| (self.sort[pos].1, pos + 1))
    }

    fn cycle_sort(&mut self, id: &str, additive: bool) {
        let current = self.sort_of(id).map(|(dir, _)| dir);
        if !additive {
            self.sort.clear();
        }
        self.sort.retain(|(cid, _)| cid != id);
        match current {
            None => self.sort.push((id.to_owned(), SortDir::Asc)),
            Some(SortDir::Asc) => self.sort.push((id.to_owned(), SortDir::Desc)),
            Some(SortDir::Desc) => {}
        }
        self.mark_dirty();
    }

    fn order<T: TableEntity>(&mut self, data: &[T], columns: &[Column<T>]) -> &[usize] {
        if self.order.is_none() {
            let query = self.search.trim().to_lowercase();
            let mut order: Vec<usize> = (0..data.len())
                .filter(|&i| {
                    query.is_empty()
                        || data[i].name().to_lowercase().contains(&query)
                        || data[i].row_id().to_lowercase().contains(&query)
                })
                .collect();

            let keys: Vec<(&Column<T>, SortDir)> = self
                .sort
                .iter()
                .filter_map(|(id, dir)| columns.iter().find(|c| c.id == id).map(|c| (c, *dir)))
                .collect();

            order.sort_by(|&a, &b| {
                for (col, dir) in &keys {
                    let ord = (col.value)(&data[a]).cmp(&(col.value)(&data[b]));
                    let ord = if *dir == SortDir::Desc {
                        ord.reverse()
                    } else {
                        ord
                    };
                    if ord != Ordering::Equal {
                        return ord;
                    }
                }
                data[a].name().cmp(data[b].name())
            });

            self.order = Some(order);
        }
        self.order.as_deref().unwrap()
    }

    pub fn shown(&self) -> Option<usize> {
        self.order.as_ref().map(|o| o.len())
    }

    pub fn sort_summary<T: TableEntity>(&self, columns: &[Column<T>]) -> Option<String> {
        if self.sort.is_empty() {
            return None;
        }
        let parts: Vec<String> = self
            .sort
            .iter()
            .filter_map(|(id, dir)| {
                let label = columns.iter().find(|c| c.id == id)?.label;
                let arrow = if *dir == SortDir::Asc { "↑" } else { "↓" };
                Some(format!("{label} {arrow}"))
            })
            .collect();
        Some(parts.join(", "))
    }
}

pub struct Table<'a, T: TableEntity> {
    id_salt: &'a str,
    data: &'a [T],
    state: &'a mut TableState,
    selected: &'a mut Option<String>,
    sprites: &'a mut SpriteCache,
}

impl<'a, T: TableEntity> Table<'a, T> {
    pub fn new(
        id_salt: &'a str,
        data: &'a [T],
        state: &'a mut TableState,
        selected: &'a mut Option<String>,
        sprites: &'a mut SpriteCache,
    ) -> Self {
        Self {
            id_salt,
            data,
            state,
            selected,
            sprites,
        }
    }

    pub fn show(self, ui: &mut Ui) {
        let columns = T::columns();
        self.state.ensure_initialized(columns);

        let visible: Vec<&Column<T>> = self
            .state
            .visible
            .iter()
            .filter_map(|id| columns.iter().find(|c| c.id == id))
            .collect();

        let order: Vec<usize> = self.state.order(self.data, columns).to_vec();

        let Self {
            id_salt,
            data,
            state,
            selected,
            sprites,
        } = self;

        let multi_sort = state.sort.len() > 1;

        ScrollArea::horizontal()
            .id_salt((id_salt, "hscroll"))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let mut builder = TableBuilder::new(ui)
                    .id_salt(id_salt)
                    .striped(true)
                    .resizable(true)
                    .sense(Sense::click())
                    .auto_shrink([false, false])
                    .min_scrolled_height(0.0)
                    .cell_layout(Layout::left_to_right(Align::Center))
                    .column(EColumn::exact(ROW_HEIGHT)); // sprite column

                for col in &visible {
                    builder = builder.column(EColumn::initial(col.width).at_least(28.0).clip(true));
                }

                builder
                    .header(HEADER_HEIGHT, |mut header| {
                        header.col(|ui| {
                            ui.add_space(2.0);
                            ui.weak(icons::IMAGE);
                        });
                        for col in &visible {
                            let sort = state.sort_of(col.id);
                            let (_rect, response) = header.col(|ui| {
                                header_label(ui, col, sort, multi_sort);
                            });
                            let response = response.on_hover_text(header_hint(col));
                            if response.clicked() {
                                let additive = response.ctx.input(|i| i.modifiers.shift);
                                state.cycle_sort(col.id, additive);
                            }
                        }
                    })
                    .body(|body| {
                        body.rows(ROW_HEIGHT, order.len(), |mut row| {
                            let entity = &data[order[row.index()]];
                            let is_selected = selected.as_deref() == Some(entity.row_id());
                            row.set_selected(is_selected);

                            row.col(|ui| {
                                show_sprite(ui, sprites, entity.sprite_path(), ROW_HEIGHT - 4.0);
                            });
                            for col in &visible {
                                row.col(|ui| {
                                    let value = (col.value)(entity);
                                    if col.numeric {
                                        ui.with_layout(
                                            Layout::right_to_left(Align::Center),
                                            |ui| {
                                                ui.monospace(value.to_string());
                                            },
                                        );
                                    } else {
                                        ui.label(value.to_string());
                                    }
                                });
                            }

                            if row.response().clicked() {
                                *selected = Some(entity.row_id().to_owned());
                            }
                        });
                    });
            });
    }
}

fn header_label<T: TableEntity>(
    ui: &mut Ui,
    col: &Column<T>,
    sort: Option<(SortDir, usize)>,
    multi_sort: bool,
) {
    let mut text = col.label.to_string();
    if let Some((dir, rank)) = sort {
        text.push(' ');
        text.push_str(match dir {
            SortDir::Asc => icons::CARET_UP,
            SortDir::Desc => icons::CARET_DOWN,
        });
        if multi_sort {
            text.push_str(&rank.to_string());
        }
    }
    let rich = if sort.is_some() {
        RichText::new(text).strong()
    } else {
        RichText::new(text)
    };
    let label = egui::Label::new(rich).selectable(false).truncate();
    if col.numeric {
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| ui.add(label));
    } else {
        ui.add(label);
    }
}

fn header_hint<T: TableEntity>(col: &Column<T>) -> String {
    let hint = "Click to sort · Shift-click to add a sort key";
    if col.tooltip.is_empty() {
        hint.to_owned()
    } else {
        format!("{}\n\n{hint}", col.tooltip)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Row {
        id: String,
        name: String,
        hp: i64,
    }

    impl TableEntity for Row {
        fn row_id(&self) -> &str {
            &self.id
        }
        fn name(&self) -> &str {
            &self.name
        }
        fn sprite_path(&self) -> Option<&Path> {
            None
        }
        fn columns() -> &'static [Column<Self>] {
            const COLS: &[Column<Row>] = &[
                Column {
                    id: "name",
                    label: "Name",
                    numeric: false,
                    default_visible: true,
                    higher_better: None,
                    width: 80.0,
                    tooltip: "",
                    value: |r| CellValue::Text(r.name.clone()),
                },
                Column {
                    id: "hp",
                    label: "HP",
                    numeric: true,
                    default_visible: true,
                    higher_better: Some(true),
                    width: 50.0,
                    tooltip: "",
                    value: |r| CellValue::Int(r.hp),
                },
            ];
            COLS
        }
    }

    fn fleet() -> Vec<Row> {
        vec![
            Row {
                id: "a".into(),
                name: "Wolf".into(),
                hp: 1000,
            },
            Row {
                id: "b".into(),
                name: "Lasher".into(),
                hp: 2000,
            },
            Row {
                id: "c".into(),
                name: "Onslaught".into(),
                hp: 2000,
            },
        ]
    }

    #[test]
    fn cellvalue_orders_numbers_with_empty_last() {
        assert_eq!(CellValue::Int(1).cmp(&CellValue::Int(2)), Ordering::Less);
        assert_eq!(
            CellValue::Float(2.5).cmp(&CellValue::Float(2.5)),
            Ordering::Equal
        );
        assert_eq!(CellValue::Int(5).cmp(&CellValue::Empty), Ordering::Less);
        assert_eq!(CellValue::Empty.cmp(&CellValue::Empty), Ordering::Equal);
        assert_eq!(CellValue::Int(3).as_f64(), Some(3.0));
        assert_eq!(CellValue::Text("x".into()).as_f64(), None);
    }

    #[test]
    fn single_column_cycles_asc_desc_then_off() {
        let mut s = TableState::default();
        s.cycle_sort("hp", false);
        assert_eq!(s.sort, vec![("hp".to_owned(), SortDir::Asc)]);
        s.cycle_sort("hp", false);
        assert_eq!(s.sort, vec![("hp".to_owned(), SortDir::Desc)]);
        s.cycle_sort("hp", false);
        assert!(s.sort.is_empty());
    }

    #[test]
    fn additive_sort_appends_then_resets() {
        let mut s = TableState::default();
        s.cycle_sort("name", false);
        s.cycle_sort("hp", true);
        assert_eq!(s.sort.len(), 2);
        assert_eq!(s.sort[0].0, "name");
        assert_eq!(s.sort[1].0, "hp");
        s.cycle_sort("hp", false);
        assert_eq!(s.sort, vec![("hp".to_owned(), SortDir::Desc)]);
    }

    #[test]
    fn order_applies_filter_then_multisort() {
        let data = fleet();
        let cols = Row::columns();
        let mut s = TableState::default();
        s.ensure_initialized(cols);

        s.cycle_sort("hp", false);
        s.cycle_sort("hp", false);
        s.cycle_sort("name", true);
        assert_eq!(s.order(&data, cols), &[1, 2, 0]);

        s.search = "wolf".into();
        s.mark_dirty();
        assert_eq!(s.order(&data, cols), &[0]);
    }

    #[test]
    fn cached_order_is_reused_until_marked_dirty() {
        let data = fleet();
        let cols = Row::columns();
        let mut s = TableState::default();
        assert!(s.order.is_none());
        s.order(&data, cols);
        assert!(s.order.is_some());
        s.mark_dirty();
        assert!(s.order.is_none());
    }
}
