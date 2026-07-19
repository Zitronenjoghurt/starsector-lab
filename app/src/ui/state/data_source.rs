use starsector_lab::validate_starsector_core_dir;
use std::path::{Path, PathBuf};

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct DataSource {
    pub core_dir: Option<PathBuf>,
    #[serde(default)]
    pub saved: Vec<PathBuf>,
}

impl DataSource {
    pub fn set(&mut self, dir: PathBuf) {
        if !self.saved.contains(&dir) {
            self.saved.push(dir.clone());
        }
        self.core_dir = Some(dir);
    }

    pub fn forget(&mut self, dir: &Path) {
        self.saved.retain(|d| d != dir);
        if self.core_dir.as_deref() == Some(dir) {
            self.core_dir = None;
        }
    }

    pub fn is_valid(&self) -> bool {
        self.core_dir
            .as_deref()
            .is_some_and(|dir| validate_starsector_core_dir(dir).is_ok())
    }
}
