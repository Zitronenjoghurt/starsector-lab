use strum::EnumIter;

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, EnumIter, serde::Serialize, serde::Deserialize,
)]
pub enum Dataset {
    #[default]
    Ships,
    Weapons,
}

impl Dataset {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Ships => "Ships",
            Self::Weapons => "Weapons",
        }
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, EnumIter, serde::Serialize, serde::Deserialize,
)]
pub enum CenterTab {
    #[default]
    Data,
    Compare,
}

impl CenterTab {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Data => "Data",
            Self::Compare => "Compare",
        }
    }
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct ViewState {
    pub dataset: Dataset,
    pub center: CenterTab,
}
