use crate::{info, success};
use flate2::bufread::GzDecoder;

use reqwest::{
    header::{HeaderMap, USER_AGENT},
    Client,
};
use serde::Deserialize;
use std::{fs, io::Read, path};

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
        OS::Mac => "oxido-darwin.zip",
        OS::Linux => "oxido-linux.tar.gz",
        OS::Windows => "oxido-windows.zip",
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
    let reader = std::io::Cursor::new(bytes);

    info!["Moving package"];

    match os {
        OS::Linux => {
            let tarfile = GzDecoder::new(reader);
            let mut archive = tar::Archive::new(tarfile);
            archive.unpack(format!(
                "{}/.oxido/bin/oxido",
                std::env::var("HOME").unwrap()
            ))?;
        }
        OS::Mac | OS::Windows => {
            let mut zip = zip::ZipArchive::new(reader).unwrap();

            let name = if os == OS::Windows {
                String::from(r"oxido.exe")
            } else {
                String::from("oxido")
            };

            let dir = if os == OS::Mac {
                format!("{}/.oxido/bin/oxido", std::env::var("HOME").unwrap())
            } else {
                String::from(r"C:\bin")
            };

            let mut file_zip = zip.by_name(&name).unwrap();
            let mut file_buf: Vec<u8> = Vec::new();
            file_zip.read_to_end(&mut file_buf)?;

            fs::create_dir_all(&dir)?;
            let path = path::Path::new(&dir).join(name);
            fs::write(path, file_buf)?;
        }
    }

    success!["Oxido has been installed!"];

    Ok(())
}
