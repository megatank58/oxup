use crate::{os::OS, success};
use std::{
    env::var,
    fs::{remove_dir_all, remove_file},
};

pub fn uninstall(os: OS) -> Result<(), Box<dyn std::error::Error>> {
    match os {
        OS::Linux | OS::Mac => {
            remove_dir_all(format!("{}/.oxido/bin/oxido", var("HOME")?))?;
        }
        OS::Windows => {
            remove_file(r"C:\bin\oxido")?;
        }
    }

    success!("Oxido has been uninstalled");

    Ok(())
}
