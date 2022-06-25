use crate::shell_command;
use regex::Regex;
use std::fs::{copy, remove_file};
pub fn install_m() {
    let mut s = shell_command(
        "curl",
        vec!["https://api.github.com/repositories/500013933/releases/latest"],
    );

    let re = Regex::new("\"browser_download_url\":.\"(?P<url>.*darwin.zip)\"").unwrap();

    s = re
        .captures(&s)
        .unwrap()
        .name("url")
        .unwrap()
        .as_str()
        .to_string();

    println!("downloading release from {}...", &s);

    shell_command("wget", vec![&s]);

    println!("unpacking release...");

    shell_command("unzip", vec!["oxido-darwin.zip"]);

    println!("moving to $HOME/.oxido/bin...");

    copy(
        "oxido",
        format!("{}/.oxido/bin/oxido", std::env::var("HOME").unwrap()),
    )
    .unwrap();
    remove_file("oxido").unwrap();
    remove_file("oxido-darwin.zip").unwrap();
}
