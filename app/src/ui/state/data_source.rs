use starsector_lab::locate::resolve_core_dir;
use starsector_lab::validate_starsector_core_dir;
use std::path::{Path, PathBuf};

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct DataSource {
    pub core_dir: Option<PathBuf>,
    #[serde(default)]
    pub saved: Vec<PathBuf>,
    #[serde(skip)]
    pub last_error: Option<String>,
}

impl DataSource {
    pub fn set(&mut self, dir: PathBuf) {
        if !self.saved.contains(&dir) {
            self.saved.push(dir.clone());
        }
        self.core_dir = Some(dir);
    }

    pub fn set_from_pick(&mut self, picked: PathBuf) -> bool {
        match resolve_core_dir(&picked) {
            Some(core) => {
                self.set(core);
                self.last_error = None;
                true
            }
            None => {
                self.last_error = Some(format!(
                    "No Starsector data found in \"{}\". Pick your Starsector \
                     install folder, Starsector.app, or the starsector-core folder.",
                    picked.display()
                ));
                false
            }
        }
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
