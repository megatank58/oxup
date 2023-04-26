use std::fs::{copy, metadata, write};

use crate::{install::install, os::OS, unset::unset, success};

pub async fn set(os: OS, mut version: String) -> Result<(), Box<dyn std::error::Error>> {
    unset(os)?;

    if !version.starts_with('v') {
        version = String::from("v") + &version;
    }

    let bin = os.path()
        + match os {
            OS::Windows => "oxido.exe",
            _ => "oxido",
        };

    let vbin = bin.clone() + "_" + &version;

    if metadata(&vbin).is_ok() {
        copy(vbin, bin)?;
    } else {
        install(os, false, Some(version.clone())).await?;
    }

    write(os.path() + ".version", &version)?;

    success!(format!("Set {version} as default"));

    Ok(())
}
