use crate::data::ship::Ship;
use crate::error::LabResult;
use crate::validate_starsector_core_dir;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use std::path::Path;
use std::path::PathBuf;

mod hull;
pub use hull::Hull;

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

    pub fn hulls(&self) -> LabResult<CsvRows<Hull>> {
        self.csv("data/hulls/ship_data.csv")
    }

    pub fn ships(&self) -> LabResult<impl Iterator<Item = LabResult<Ship>>> {
        Ok(self.hulls()?.filter_map(|res| match res {
            Ok(hull) => Ship::from_hull(hull).map(Ok),
            Err(e) => Some(Err(e)),
        }))
    }
}

pub struct CsvRows<T> {
    reader: csv::Reader<std::fs::File>,
    headers: csv::StringRecord,
    record: csv::StringRecord,
    _marker: PhantomData<T>,
}

impl<T> CsvRows<T> {
    pub fn open(path: impl AsRef<Path>) -> LabResult<Self> {
        let mut reader = csv::ReaderBuilder::new()
            .comment(Some(b'#'))
            .flexible(true)
            .trim(csv::Trim::All)
            .from_path(path)?;
        let headers = reader.headers()?.clone();
        Ok(Self {
            reader,
            headers,
            record: csv::StringRecord::new(),
            _marker: PhantomData,
        })
    }
}

impl<T: DeserializeOwned> Iterator for CsvRows<T> {
    type Item = LabResult<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            return match self.reader.read_record(&mut self.record) {
                Ok(false) => None,
                Err(e) => Some(Err(e.into())),
                Ok(true) => {
                    if self.record.iter().all(|f| f.trim().is_empty()) {
                        continue;
                    }
                    Some(
                        self.record
                            .deserialize(Some(&self.headers))
                            .map_err(Into::into),
                    )
                }
            };
        }
    }
}
