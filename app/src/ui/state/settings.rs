use crate::ui::background::BackgroundSettings;
use strum::EnumIter;

#[derive(Default, Copy, Clone, PartialEq, Eq, EnumIter, serde::Serialize, serde::Deserialize)]
pub enum SettingsTab {
    #[default]
    General,
    Background,
}

impl SettingsTab {
    pub fn label(&self) -> &'static str {
        match self {
            Self::General => "General",
            Self::Background => "Background",
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Settings {
    pub ui_scale: f32,
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
