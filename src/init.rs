use std::fs::{create_dir_all, write};

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
");

    create_dir_all(format!("{name}/src")).unwrap();
    write(format!("{name}/src/main.ox"), default_function).unwrap();
    write(format!("{name}/Oxup.toml"), metadata).unwrap();

    success![format!("Successfully initialised {name}!")]
}
