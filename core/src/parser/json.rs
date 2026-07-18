use crate::error::LabResult;
use serde::de::DeserializeOwned;
use std::path::Path;

pub mod ship;

pub fn read_json<T: DeserializeOwned>(path: impl AsRef<Path>) -> LabResult<T> {
    let text = std::fs::read_to_string(path)?;
    Ok(json5::from_str(&text)?)
}
