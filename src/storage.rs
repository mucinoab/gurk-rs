use log::info;
use serde::{de::DeserializeOwned, Serialize};

use std::fs::File;
use std::path::PathBuf;

/// Trait for persisting data.
///
/// This trait is too simple atm. In the future, we will make it more granular to support storing
/// different types of data independently of each other.
pub trait Storage {
    fn load<D: DeserializeOwned>(&self) -> anyhow::Result<D>;
    fn save<D: Serialize>(&self, data: &D) -> anyhow::Result<()>;
}

pub struct StorageMock {}

impl StorageMock {
    pub fn new() -> Self {
        Self {}
    }
}

impl Storage for StorageMock {
    fn load<D: DeserializeOwned>(&self) -> anyhow::Result<D> {
        todo!()
    }

    fn save<D: Serialize>(&self, _data: &D) -> anyhow::Result<()> {
        Ok(())
    }
}

pub struct JsonStorage {
    path: PathBuf,
}

impl JsonStorage {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Storage for JsonStorage {
    fn load<D: DeserializeOwned>(&self) -> anyhow::Result<D> {
        info!("loading app data from: {}", self.path.display());
        let f = std::io::BufReader::new(File::open(&self.path)?);
        Ok(serde_json::from_reader(f)?)
    }

    fn save<D: Serialize>(&self, data: &D) -> anyhow::Result<()> {
        let f = std::io::BufWriter::new(File::create(&self.path)?);
        serde_json::to_writer(f, data)?;
        Ok(())
    }
}
