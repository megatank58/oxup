use crate::{info, success};
use reqwest::{
    header::{HeaderMap, USER_AGENT},
    Client,
};
use serde::Deserialize;
use std::{
    fs::{set_permissions, write, Permissions},
    os::unix::prelude::PermissionsExt,
};

use crate::os::OS;

#[derive(Deserialize, Debug)]
pub struct Release {
    browser_download_url: String,
}

#[derive(Deserialize, Debug)]
pub struct ReleaseData {
    name: String,
    assets: Vec<Release>,
}

pub async fn install(os: OS, oxate: bool) -> Result<(), Box<dyn std::error::Error>> {
    let target = if oxate {
        "https://api.github.com/repos/oxidic/oxate/releases/latest"
    } else {
        "https://api.github.com/repositories/500013933/releases/latest"
    };
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "megatank58".parse().unwrap());
    let response = client.get(target).headers(headers.clone()).send().await?;
    let result: ReleaseData = response.json().await?;

    let bin = if oxate { "oxate" } else { "oxido" };

    let filter = &match os {
        OS::Mac => bin.to_owned() + "darwin",
        OS::Linux => bin.to_owned(),
        OS::Windows => bin.to_owned() + ".exe",
    };

    let url: String = result
        .assets
        .iter()
        .filter(|f| f.browser_download_url.ends_with(filter))
        .map(|f| f.browser_download_url.clone())
        .collect();

    info!(format!("Downloading {bin}"));

    let response = client.get(&url).headers(headers.clone()).send().await?;
    let bytes = response.bytes().await?;

    info!("Moving package");

    write(
        os.path()
            + &match os {
                OS::Windows => bin.to_string() + ".exe",
                _ => bin.to_string(),
            },
        bytes,
    )?;

    if os == OS::Linux || os == OS::Mac {
        set_permissions(
            os.path() + bin,
            Permissions::from_mode(0o770),
        )?;
    }

    write(os.path() + ".version", result.name)?;

    success!(format!("{bin} has been installed!"));

    Ok(())
}
