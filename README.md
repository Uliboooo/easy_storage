# Storeable Crate

A Rust crate that provides a trait for easily saving and loading data structures to and from files in either JSON or TOML format.

## Installation

Add the following to your `Cargo.toml` file:

```toml: Cargo.toml
[dependencies]
easy_storage = "0.1.0"
```

## Example Usage

```rust
use serde::{Serialize, Deserialize};
use storeable::{Storeable, Format, Error};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct MyData {
    name: String,
    value: i32,
}

impl Storeable<Path> for MyData {}

fn main() -> Result<(), Error> {
    let data = MyData {
        name: "Example".to_string(),
        value: 42,
    };

    let path = Path::new("data.json");

    // Save to file
    data.save(path, true, Format::Json)?; // Creates a new file if it doesn't exist

    // Load from file
    let loaded_data = MyData::load(path, Format::Json)?;

    assert_eq!(data, loaded_data);
    println!("Loaded data: {:?}", loaded_data);

    // Clean up the file (optional)
    std::fs::remove_file(path).unwrap();

    Ok(())
}
```
