use std::path::Path;

pub mod data;
mod error;
pub mod locate;
pub mod parser;

pub fn validate_starsector_core_dir(dir: &Path) -> Result<(), error::LabError> {
    for sub in ["data", "graphics", "sounds"] {
        if !dir.join(sub).is_dir() {
            return Err(error::LabError::InvalidDataDir(dir.to_owned()));
        }
    }
    Ok(())
}
