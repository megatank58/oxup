use std::fs::read;

use reqwest::{
    header::{HeaderMap, USER_AGENT},
    Client,
};
use serde::Deserialize;

use crate::{os::OS, success};

#[derive(Deserialize, Debug)]
pub struct Release {
    name: String,
}

pub async fn list(os: OS) {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "megatank58".parse().unwrap());
    let response = client
        .get("https://api.github.com/repos/oxidic/oxido/releases")
        .headers(headers.clone())
        .send()
        .await
        .unwrap();
    let releases: Vec<Release> = response.json().await.unwrap();

    let bytes = match read(os.path() + ".version") {
        Ok(b) => b,
        _ => vec![]
    };
    let current = String::from_utf8(bytes).unwrap();

    success!("Oxido versions:");
    for release in &releases {
        if current.trim() == release.name {
            println!("   \x1b[1m{} * default\x1b[0m", release.name);
        } else {
            println!("   \x1b[1m{}\x1b[0m", release.name);
        }
    }

    if current < releases.first().unwrap().name {
        println!("\nupdate to {r} by running \x1b[1m`oxate install {r}`\x1b[0m", r=releases.first().unwrap().name);
    }
}
