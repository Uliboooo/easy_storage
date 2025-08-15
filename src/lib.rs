//! # Examples
//!
//! ```
//! use serde::{Deserialize, Serialize};
//! use easy_storage::Storeable;
//!
//! #[derive(Debug, Serialize, Deserialize)]
//! struct User {
//!     name: String,
//!     email: String,
//! }
//!
//! impl Storeable for User {}
//!
//! fn main() {
//!     let user = User {
//!         name: "Alice".to_string(),
//!         email: "alice@alice.com".to_string(),
//!     };
//!     let save_path = std::env::current_dir().unwrap().join("test").join("user.toml");
//!     match user.save_by_extension(&save_path, true) {
//!         Ok(_) => println!("success."),
//!         Err(e) => println!("Error: {e}"),
//!     }
//!
//!     match User::load_by_extension(save_path) {
//!         Ok(s) => println!("{s:?}"),
//!         Err(e) => println!("Error: {e}"),
//!     }
//! }
//! ```

use serde::{Serialize, de::DeserializeOwned};
use std::{fmt::Display, fs::OpenOptions, io::Write, path::Path};

#[derive(Debug)]
pub enum Error {
    IoE(std::io::Error),
    JsonE(serde_json::Error),
    ParTomlE(toml::ser::Error),
    DesTomlE(toml::de::Error),
    ExtensionDoesNotExist,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoE(value)
    }
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
            Error::ExtensionDoesNotExist => write!(f, "extension does not exist."),
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
            Error::ExtensionDoesNotExist => None,
        }
    }
}

pub enum Format {
    Json,
    Toml,
}

fn path_to_format<P: AsRef<Path>>(path: P) -> Result<Format, Error> {
    if let Some(v) = path.as_ref().extension().map(|f| f.to_str()).flatten() {
        match v {
            "json" => Ok(Format::Json),
            "toml" => Ok(Format::Toml),
            _ => Err(Error::ExtensionDoesNotExist),
        }
    } else {
        Err(Error::ExtensionDoesNotExist)
    }
}

pub trait Storeable: Serialize + DeserializeOwned + Sized {
    /// Save to file.
    ///
    /// # Arguments
    /// * `path` - A string slice that holds the path to the file.
    /// * `new_create` - A boolean that indicates whether to create a new file if it does not exist.
    /// * `format` - A `Format` enum that indicates the format to save the file in.
    ///
    /// # Returns
    /// * `Result<(), Error>` - A `Result` enum that indicates whether the operation was successful.
    fn save<P: AsRef<Path>>(&self, path: P, new_create: bool, format: Format) -> Result<(), Error> {
        let s = match format {
            Format::Json => serde_json::to_string_pretty(self)?,
            Format::Toml => toml::to_string_pretty(self)?,
        };
        // let s = toml::to_string_pretty(self).map_err(Error::ParTomlE)?;
        let mut f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(new_create)
            .open(path)?;
        // .map_err(Error::IoE)?;

        f.write_all(s.as_bytes())?;
        Ok(())
    }

    /// save to file by extension of `path`
    ///
    /// supported extensions are `json` and `toml`.
    ///
    /// # Arguments
    /// * `path` - path to the file.
    /// * `new_create` - a boolean that indicates whether to create a new if it does not exist.
    ///
    /// # Returns
    /// * `Result<(), Error>` - return errors if path does not include extension or include a not-supported extension or others reasons(io, fs, json(toml) parse).
    fn save_by_extension<P: AsRef<Path>>(&self, path: P, new_create: bool) -> Result<(), Error> {
        let format = path_to_format(&path)?;
        self.save(path, new_create, format)
    }

    /// Load from file.
    ///
    /// # Arguments
    /// * `path` - A string slice that holds the path to the file.
    /// * `format` - A `Format` enum that indicates the format to load the file from.
    ///
    /// # Returns
    /// * `Result<Self, Error>` - A `Result` enum that indicates whether the operation was successful.
    fn load<P: AsRef<Path>>(path: P, format: Format) -> Result<Self, Error> {
        let content = std::fs::read_to_string(path)?;
        // return deserialized date
        Ok(match format {
            Format::Json => serde_json::from_str::<Self>(&content)?,
            Format::Toml => toml::from_str::<Self>(&content)?,
        })
    }

    /// load from file by extension of `path`
    ///
    /// supported extensions are `json` and `toml`
    ///
    /// # Arguments
    ///
    /// * `path` - path to load file.
    ///
    /// # Returns
    /// * `Result<(), Error>` - return errors if path does not include extension or include a not-supported exttension or others reasons(io, fs, json(toml) parse).
    fn load_by_extension<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let format = path_to_format(&path)?;
        Self::load(path, format)
    }
}

#[cfg(test)]
mod tests {
    use crate::Storeable;
    use serde::{Deserialize, Serialize};
    use std::path::PathBuf;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct User {
        name: String,
        email: String,
    }

    impl Storeable for User {}

    fn ready_test_env() -> PathBuf {
        let test_path = std::env::current_dir().unwrap().join("test");
        if !test_path.exists() {
            std::fs::create_dir_all(&test_path).unwrap();
        }
        test_path
    }

    #[test]
    fn save_test() {
        let test_f_path = ready_test_env();
        let save_path = test_f_path.join("user.toml");
        let user = User {
            name: "Alice".to_string(),
            email: "alice@alice.com".to_string(),
        };
        let res = user.save_by_extension(save_path, true);
        if let Err(e) = &res {
            eprintln!("{e}");
        }
        assert!(res.is_ok());
    }

    #[test]
    fn load_test() {
        let test_f_path = ready_test_env();
        let save_path = test_f_path.join("user.toml");
        let user = User {
            name: "Alice".to_string(),
            email: "alice@alice.com".to_string(),
        };

        user.save_by_extension(&save_path, true).unwrap();

        let loaded = User::load_by_extension(save_path);
        match loaded {
            Ok(v) => assert_eq!(v, user),
            Err(_) => assert!(loaded.is_ok()),
        }
    }
}
