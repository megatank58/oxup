use crate::{info, shell, success};
use colored::Colorize;
use regex::Regex;

pub fn install_w() {
    let mut s = shell!(
        "curl",
        vec!["https://api.github.com/repositories/500013933/releases/latest"]
    );

    let re = Regex::new("\"browser_download_url\":.\"(?P<url>.*gnu.zip)\"").unwrap();
    let vre = Regex::new(r"v(?P<version>\d.\d.\d)").unwrap();

    s = re
        .captures(&s)
        .unwrap()
        .name("url")
        .unwrap()
        .as_str()
        .to_string();

    let v = vre
        .captures(&s)
        .unwrap()
        .name("version")
        .unwrap()
        .as_str()
        .to_string();

    info![format!("Downloading oxido version {}", &v)];

    shell!("wget", vec![&s]);

    info!["Unpacking package"];

    shell!("unzip", vec!["oxido-windows.gnu.zip"]);

    info!["Moving package"];

    shell!("mkdir", vec![r"C:\bin"]);

    shell!("move", vec![r"oxido.exe C:\oxido"]);

    success!["Oxido has been installed!"];

    shell!("setx", vec!["PATH \"C:\\oxido;%PATH%\""]);
}
