use std::fs::{metadata, remove_file};

use crate::os::OS;

pub fn unset(os: OS) -> Result<(), Box<dyn std::error::Error>> {
    let bin = os.path()
        + match os {
            OS::Windows => "oxido.exe",
            _ => "oxido",
        };
    if metadata(&bin).is_ok() {
        remove_file(bin)?;
    }
    if metadata(os.path() + ".version").is_ok() {
        remove_file(os.path() + ".version")?;
    }

    Ok(())
}
