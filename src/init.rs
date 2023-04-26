use std::fs::{create_dir_all, write};

use crate::success;

pub fn init(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let default_function = "println(\"Hello world!\");";
    let metadata = format!(
        "[package]
name = \"{name}\"
version = \"0.1.0\"
"
    );

    create_dir_all(format!("{name}/src"))?;
    write(format!("{name}/src/main.oxi"), default_function)?;
    write(format!("{name}/Oxate.toml"), metadata)?;

    success!(format!("Successfully initialised {name}!"));

    Ok(())
}
