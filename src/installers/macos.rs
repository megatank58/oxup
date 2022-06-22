use crate::{shell_command};
use regex::Regex;

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

    println!("Downloading release from {}...", &s);

    shell_command("wget", vec![&s]);

    println!("Unpacking release...");

    shell_command("unzip", vec!["oxido*.darwin.zip"]);

    println!("Moving to $HOME/.oxido...");

    shell_command("mkdir", vec!["$HOME/.oxido"]);
    shell_command("mv", vec!["oxido* $HOME/.oxido"]);

    println!("Oxup installed successfully!\nRun echo \"export PATH=\"$HOME/.oxido:$PATH\"\" >> $HOME/.bashrc and restart your terminal to use it");
}
