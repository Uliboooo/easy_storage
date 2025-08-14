use serde::{Serialize, de::DeserializeOwned};
use std::{fmt::Display, fs::OpenOptions, io::Write, path::Path};

#[derive(Debug)]
pub enum Error {
    IoE(std::io::Error),
    JsonE(serde_json::Error),
    ParTomlE(toml::ser::Error),
    DesTomlE(toml::de::Error),
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonE(value)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(value: toml::ser::Error) -> Self {
        Self::ParTomlE(value)
    }
}

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Self::DesTomlE(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoE(e) => write!(f, "{e}"),
            Error::JsonE(e) => write!(f, "{e}"),
            Error::ParTomlE(e) => write!(f, "{e}"),
            Error::DesTomlE(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IoE(e) => Some(e),
            Error::JsonE(e) => Some(e),
            Error::ParTomlE(e) => Some(e),
            Error::DesTomlE(e) => Some(e),
        }
    }
}

pub enum Format {
    Json,
    Toml,
}

pub trait Storeable<P: AsRef<Path>> {
    /// Save to file.
    ///
    /// # Arguments
    /// * `path` - A string slice that holds the path to the file.
    /// * `new_create` - A boolean that indicates whether to create a new file if it does not exist.
    /// * `format` - A `Format` enum that indicates the format to save the file in.
    ///
    /// # Returns
    /// * `Result<(), Error>` - A `Result` enum that indicates whether the operation was successful.
    fn save(&self, path: P, new_create: bool, format: Format) -> Result<(), Error>
    where
        Self: Serialize + DeserializeOwned,
    {
        let s = match format {
            Format::Json => serde_json::to_string_pretty(self).map_err(Error::JsonE),
            Format::Toml => toml::to_string_pretty(self).map_err(Error::ParTomlE),
        }?;
        // let s = toml::to_string_pretty(self).map_err(Error::ParTomlE)?;
        let mut f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(new_create)
            .open(path)
            .map_err(Error::IoE)?;

        f.write_all(s.as_bytes()).map_err(Error::IoE)
    }

    /// Load from file.
    ///
    /// # Arguments
    /// * `path` - A string slice that holds the path to the file.
    /// * `format` - A `Format` enum that indicates the format to load the file from.
    ///
    /// # Returns
    /// * `Result<Self, Error>` - A `Result` enum that indicates whether the operation was successful.
    fn load(path: P, format: Format) -> Result<Self, Error>
    where
        Self: Serialize + DeserializeOwned,
    {
        let content = std::fs::read_to_string(path).map_err(Error::IoE)?;
        // return deserialized date
        match format {
            Format::Json => serde_json::from_str::<Self>(&content).map_err(Error::JsonE),
            Format::Toml => toml::from_str::<Self>(&content).map_err(Error::DesTomlE),
        }
    }
}
