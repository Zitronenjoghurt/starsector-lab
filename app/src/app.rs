use crate::ui;
use crate::ui::state::UiState;
use crate::ui::theme::Theme;
use crate::ui::widgets::sprite::SpriteCache;
use eframe::{Frame, Storage};
use egui::Ui;
use starsector_lab::data::Data;
use starsector_lab::parser::Parser;

#[derive(Default)]
pub enum LoadStatus {
    #[default]
    Idle,
    Loaded {
        ships: usize,
        weapons: usize,
    },
    Failed(String),
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Lab {
    pub ui: UiState,
    #[serde(default, skip)]
    pub data: Data,
    #[serde(default, skip)]
    pub status: LoadStatus,
    #[serde(default, skip)]
    pub sprites: SpriteCache,
}

impl Lab {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.5);
        Theme::apply(&cc.egui_ctx);
        let mut lab = cc
            .storage
            .and_then(|storage| eframe::get_value::<Self>(storage, eframe::APP_KEY))
            .unwrap_or_default();
        lab.reload_data();
        lab
    }

    pub fn reload_data(&mut self) {
        let Some(dir) = self.ui.data_source.core_dir.clone() else {
            return;
        };

        match Parser::new(&dir).and_then(|parser| parser.data()) {
            Ok(data) => {
                self.status = LoadStatus::Loaded {
                    ships: data.ships.len(),
                    weapons: data.weapons.len(),
                };
                self.data = data;
            }
            Err(err) => {
                self.status = LoadStatus::Failed(err.to_string());
                self.data = Data::default();
            }
        }

        self.ui.on_data_reloaded();
    }
}

impl eframe::App for Lab {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        ui::ui(ui, self);
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
