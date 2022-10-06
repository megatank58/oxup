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

pub async fn install(os: OS) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "megatank58".parse().unwrap());

    let target = "https://api.github.com/repositories/500013933/releases/latest";
    let client = Client::new();
    let response = client.get(target).headers(headers.clone()).send().await?;
    let result: ReleaseData = response.json().await?;

    let filter = match os {
        OS::Mac => "oxido-darwin",
        OS::Linux => "oxido",
        OS::Windows => "oxido.exe",
    };

    let url: String = result
        .assets
        .iter()
        .filter(|f| f.browser_download_url.ends_with(filter))
        .map(|f| f.browser_download_url.clone())
        .collect();

    info!["Downloading oxido"];

    let response = client.get(&url).headers(headers.clone()).send().await?;
    let bytes = response.bytes().await?;

    info!["Moving package"];

    std::fs::write(
        match os {
            OS::Windows => String::from(r"C:\bin"),
            _ => {
                format!("{}/.oxido/bin", std::env::var("HOME").unwrap())
            }
        },
        bytes,
    )?;

    success!["Oxido has been installed!"];

    Ok(())
}
