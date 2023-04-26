use crate::{error, info, success};
use reqwest::{
    header::{HeaderMap, USER_AGENT},
    Client,
};
use serde::Deserialize;
use std::{
    fs::{create_dir_all, metadata, set_permissions, write, Permissions},
    os::unix::prelude::PermissionsExt,
};

use crate::os::OS;

#[derive(Deserialize, Debug)]
pub struct Release {
    browser_download_url: String,
}

#[derive(Deserialize, Debug)]
pub struct ReleaseData {
    pub name: String,
    pub assets: Vec<Release>,
}

pub async fn install(
    os: OS,
    oxate: bool,
    version: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "megatank58".parse().unwrap());

    let release = if version.as_ref().is_none() {
        let target = if oxate {
            "https://api.github.com/repos/oxidic/oxate/releases/latest"
        } else {
            "https://api.github.com/repositories/500013933/releases/latest"
        };

        let response = client.get(target).headers(headers.clone()).send().await?;
        let result: ReleaseData = response.json().await?;

        result
    } else {
        let version = version.unwrap();
        let response = client
            .get("https://api.github.com/repos/oxidic/oxido/releases")
            .headers(headers.clone())
            .send()
            .await
            .unwrap();
        let releases: Vec<ReleaseData> = response.json().await.unwrap();

        let mut releasedata: Option<ReleaseData> = None;

        for release in releases {
            if release.name == version {
                releasedata = Some(release);
                break;
            }
        }

        if releasedata.is_none() {
            error!(format!(
                "could not find version {version}, run `list` to find a version"
            ));
        }

        releasedata.unwrap()
    };

    let bin = if oxate { "oxate" } else { "oxido" };

    let filter = &match os {
        OS::Mac => bin.to_owned() + "darwin",
        OS::Linux => bin.to_owned(),
        OS::Windows => bin.to_owned() + ".exe",
    };

    let url: String = release
        .assets
        .iter()
        .filter(|f| f.browser_download_url.ends_with(filter))
        .map(|f| f.browser_download_url.clone())
        .collect();

    info!(format!("Downloading {bin} {}", release.name));

    let response = client.get(&url).headers(headers.clone()).send().await?;
    let bytes = response.bytes().await?;

    info!(format!("Moving package to `{}`", os.path()));

    create_dir_all(os.path()).unwrap();

    let bin_exists = metadata(os.path() + bin).is_ok();

    write(
        os.path()
            + &match os {
                OS::Windows => bin.to_string() + "_" + &release.name + ".exe",
                _ => bin.to_string() + "_" + &release.name,
            },
        &bytes,
    )?;

    if !bin_exists {
        write(
            os.path()
                + &match os {
                    OS::Windows => bin.to_string() + ".exe",
                    _ => bin.to_string(),
                },
            &bytes,
        )?;
    }

    if os == OS::Linux || os == OS::Mac {
        set_permissions(
            os.path() + bin + "_" + &release.name,
            Permissions::from_mode(0o770),
        )?;
        set_permissions(os.path() + bin, Permissions::from_mode(0o770))?;
    }

    write(os.path() + ".version", release.name)?;

    success!(format!("{bin} installed successfully"));

    Ok(())
}
