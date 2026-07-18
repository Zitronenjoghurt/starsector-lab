use crate::data::ship::Ship;
use crate::data::Data;
use crate::error::LabResult;
use crate::parser::csv::hull::HullRow;
use crate::parser::csv::CsvRows;
use crate::parser::json::read_json;
use crate::parser::json::ship::ShipFile;
use crate::validate_starsector_core_dir;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::path::PathBuf;

pub mod csv;
pub mod json;

pub struct Parser {
    core_dir: PathBuf,
}

impl Parser {
    pub fn new(core_dir: impl Into<PathBuf>) -> LabResult<Self> {
        let core_dir = core_dir.into();
        validate_starsector_core_dir(&core_dir)?;
        Ok(Self { core_dir })
    }

    fn csv<T: DeserializeOwned>(&self, rel: &str) -> LabResult<CsvRows<T>> {
        CsvRows::open(self.core_dir.join(rel))
    }

    pub fn data(&self) -> LabResult<Data> {
        let ships = self.ships()?.collect::<LabResult<Vec<_>>>()?;
        Ok(Data { ships })
    }

    pub fn hulls(&self) -> LabResult<CsvRows<HullRow>> {
        self.csv("data/hulls/ship_data.csv")
    }

    pub fn ship_files(&self) -> LabResult<HashMap<String, ShipFile>> {
        let mut index = HashMap::new();
        for entry in std::fs::read_dir(self.core_dir.join("data/hulls"))? {
            let path = entry?.path();
            if path.extension().and_then(|e| e.to_str()) != Some("ship") {
                continue;
            }
            let file: ShipFile = read_json(&path)?;
            index.insert(file.hull_id.clone(), file);
        }
        Ok(index)
    }

    pub fn ships(&self) -> LabResult<impl Iterator<Item = LabResult<Ship>>> {
        let mut layouts = self.ship_files()?;
        let core_dir = self.core_dir.clone();
        Ok(self.hulls()?.filter_map(move |res| {
            let hull = match res {
                Ok(hull) => hull,
                Err(e) => return Some(Err(e)),
            };
            let layout = layouts.remove(&hull.id)?;
            Ship::from_parts(hull, layout, &core_dir).map(Ok)
        }))
    }
}
