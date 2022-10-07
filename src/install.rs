use crate::{info, success};
use reqwest::{
    header::{HeaderMap, USER_AGENT},
    Client,
};
use serde::Deserialize;

use crate::os::OS;

#[derive(Deserialize, Debug)]
pub struct Release {
    browser_download_url: String,
}

#[derive(Deserialize, Debug)]
pub struct ReleaseData {
    assets: Vec<Release>,
}

pub async fn install(os: OS, oxup: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "megatank58".parse().unwrap());

    let target = if oxup {
        "https://api.github.com/repos/oxidic/oxup/releases/latest"
    } else {
        "https://api.github.com/repositories/500013933/releases/latest"
    };
    let client = Client::new();
    let response = client.get(target).headers(headers.clone()).send().await?;
    let result: ReleaseData = response.json().await?;

    let bin = if oxup {
        "oxup"
    } else {
        "oxido"
    };

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

    info![format!("Downloading {bin}")];

    let response = client.get(&url).headers(headers.clone()).send().await?;
    let bytes = response.bytes().await?;

    info!["Moving package"];

    std::fs::write(
        match os {
            OS::Windows => format!(r"C:\bin\{bin}.exe"),
            _ => {
                format!("{}/.oxido/bin/{bin}", std::env::var("HOME").unwrap())
            }
        },
        bytes,
    )?;

    success![format!("{bin} has been installed!")];

    Ok(())
}
