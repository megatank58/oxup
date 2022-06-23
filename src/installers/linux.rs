use crate::shell_command;
use regex::Regex;
use std::fs::{copy, remove_file};

pub fn install_l() {
    let mut s = shell_command(
        "curl",
        vec!["https://api.github.com/repositories/500013933/releases/latest"],
    );

    let re = Regex::new("\"browser_download_url\":.\"(?P<url>.*tar.gz)\"").unwrap();

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

    shell_command("tar", vec!["-xf", "oxido-linux.tar.gz"]);

    println!("moving to $HOME/.oxido/bin...");

    copy(
        "oxido",
        format!("{}/.oxido/bin/oxido", std::env::var("HOME").unwrap()),
    )
    .unwrap();

    remove_file("oxido").unwrap();
    remove_file("oxido-linux.tar.gz").unwrap();
}
