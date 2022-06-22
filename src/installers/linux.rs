use std::fs::{copy, metadata, remove_file, create_dir};
use crate::shell_command;
use regex::Regex;

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

    println!("Downloading release from {}...", &s);

    shell_command("wget", vec![&s]);

    println!("Unpacking release...");

    shell_command("tar", vec!["-xf", "oxido-linux.tar.gz"]);

    println!("Moving to $HOME/.oxido...");

    if !metadata(format!("{}/.oxido", std::env::var("HOME").unwrap())).is_ok() {
        create_dir(format!("{}/.oxido", std::env::var("HOME").unwrap())).unwrap();
    }
    copy(
        "oxido",
        format!("{}/.oxido/oxido", std::env::var("HOME").unwrap()),
    )
    .unwrap();
    remove_file("oxido").unwrap();
    remove_file("oxido-linux.tar.gz").unwrap();

    println!("Oxup installed successfully!\nRun 'echo \"export PATH=\"$HOME/.oxido:$PATH\"\" >> $HOME/.bashrc' and restart your terminal to use it");
}
