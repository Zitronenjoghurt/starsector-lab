use std::path::PathBuf;

pub type LabResult<T> = Result<T, LabError>;

#[derive(Debug, thiserror::Error)]
pub enum LabError {
    #[error(transparent)]
    Csv(#[from] csv::Error),
    #[error("Invalid Starsector data directory: {0:?}")]
    InvalidDataDir(PathBuf),
}
