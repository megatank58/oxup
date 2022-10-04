use std::fs::{create_dir, write};

use crate::success;

pub fn init(name: String) {
    let default_function = 
"fn main() {
    print(\"Hello world!\");
}
";
    let metadata = format!(
"[package]
name = {name}
version = 0.1.0
"
    );

    create_dir("src").unwrap();
    write("src/main.ox", default_function).unwrap();
    write("Oxup.toml", metadata).unwrap();

    success![format!("Successfully initialised {name}!")]
}
