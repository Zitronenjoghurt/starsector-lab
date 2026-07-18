use crate::error::LabResult;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use std::path::Path;

pub mod hull;

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
