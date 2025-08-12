use serde::{Serialize, de::DeserializeOwned};
use std::{fs::OpenOptions, io::Write, path::Path};
use toml::{from_str, to_string_pretty};

#[derive(Debug)]
pub enum Error {
    IoE(std::io::Error),
    JsonE(serde_json::Error)
    ParTomlE(toml::ser::Error),
    DesTomlE(toml::de::Error),
}

pub enum Format {
    Json,
    Toml,
}

//#[cfg(feature = "toml")]
pub trait Storeable<P: AsRef<Path>> {
    fn save(&self, path: P, new_create: bool, format: Format) -> Result<(), Error>
    where
        Self: Serialize + DeserializeOwned,
    {
        let s = match format {
            Format::Json => serde_json::to_string_pretty(self).map_err(Error::JsonE),
            Format::Toml => toml::to_string_pretty(self).map_err(Error::ParTomlE),
        }?;
        //let s = toml::to_string_pretty(self).map_err(Error::ParTomlE)?;
        let mut f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(new_create)
            .open(path)
            .map_err(Error::IoE)?;
        f.write_all(s.as_bytes()).map_err(Error::IoE)
    }

    fn load(path: P, format: Format) -> Result<Self, Error>
    where
        Self: Serialize + DeserializeOwned,
    {
        let content = std::fs::read_to_string(path).map_err(Error::IoE)?;
        match format {
            Format::Json => serde_json::from_str::<Self>(&content).map_err(Error::JsonE),
            Format::Toml => toml::from_str::<Self>(&content).map_err(Error::DesTomlE),
        }
    }
}
