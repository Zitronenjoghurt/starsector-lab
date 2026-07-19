use crate::ui::background::BackgroundSettings;
use strum::EnumIter;

#[derive(Default, Copy, Clone, PartialEq, Eq, EnumIter, serde::Serialize, serde::Deserialize)]
pub enum SettingsTab {
    #[default]
    General,
    Layout,
    Background,
}

impl SettingsTab {
    pub fn label(&self) -> &'static str {
        match self {
            Self::General => "General",
            Self::Layout => "Layout",
            Self::Background => "Background",
        }
    }
}

#[derive(Copy, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LayoutSettings {
    pub table_row_height: f32,
    pub compare_column_width: f32,
    pub inspector_sprite_size: f32,
}

impl Default for LayoutSettings {
    fn default() -> Self {
        Self {
            table_row_height: 26.0,
            compare_column_width: 150.0,
            inspector_sprite_size: 180.0,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Settings {
    pub ui_scale: f32,
    #[serde(default)]
    pub layout: LayoutSettings,
    #[serde(default)]
    pub background: BackgroundSettings,
    #[serde(skip)]
    pub active_tab: SettingsTab,
    #[serde(skip, default = "default_true")]
    pub dirty: bool,
}

fn default_true() -> bool {
    true
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            ui_scale: Self::DEFAULT_UI_SCALE,
            layout: LayoutSettings::default(),
            background: BackgroundSettings::default(),
            active_tab: SettingsTab::default(),
            dirty: true,
        }
    }
}

impl Settings {
    pub const DEFAULT_UI_SCALE: f32 = 1.5;

    pub fn apply(&mut self, ctx: &egui::Context) {
        if !self.dirty {
            return;
        }

        ctx.set_pixels_per_point(self.ui_scale);

        self.dirty = false;
    }
}
