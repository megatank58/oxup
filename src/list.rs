use std::fs::read;

use reqwest::{
    header::{HeaderMap, USER_AGENT},
    Client,
};

use crate::{install::ReleaseData, os::OS, success};

pub async fn list(os: OS) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "megatank58".parse()?);
    let response = client
        .get("https://api.github.com/repos/oxidic/oxido/releases")
        .headers(headers.clone())
        .send()
        .await
        ?;
    let releases: Vec<ReleaseData> = response.json().await?;

    let bytes = match read(os.path() + ".version") {
        Ok(b) => b,
        _ => vec![],
    };
    let current = String::from_utf8(bytes)?;

    success!("Oxido versions:");
    for release in &releases {
        if current.trim() == release.name {
            println!("   \x1b[1m{} *\x1b[0m", release.name);
        } else {
            println!("   \x1b[1m{}\x1b[0m", release.name);
        }
    }

    if current < releases.first().unwrap().name {
        println!(
            "\nupdate to {r} by running \x1b[1m`oxate install {r}`\x1b[0m",
            r = releases.first().unwrap().name
        );
    };

    Ok(())
}
