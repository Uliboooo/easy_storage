# Storeable Crate

A Rust crate that provides a trait for easily saving and loading data structures to and from files in either JSON or TOML format.

## Installation

Add the following to your `Cargo.toml` file:

```toml: Cargo.toml
[dependencies]
easy_storage = "0.2.*"
```

## Example

```rust
use serde::{Deserialize, Serialize};
use easy_storage::Storeable;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
}

impl<P: AsRef<std::path::Path>> Storeable<P> for User {}

fn main() {
    let user = User {
        name: "Alice".to_string(),
        email: "alice@alice.com".to_string(),
    };
    let save_path = std::env::current_dir().unwrap().join("test").join("user.toml");
    match user.save_by_extension(&save_path, true) {
        Ok(_) => println!("success."),
        Err(e) => println!("Error: {e}"),
    }

    match User::load_by_extension(save_path) {
        Ok(s) => println!("{s:?}"),
        Err(e) => println!("Error: {e}"),
    }
}
```
