use regex::Regex;
use crate::shell_command;

pub fn install_l() {

    let mut s = shell_command(
        "curl",
        vec![
            "https://api.github.com/repositories/500013933/releases/latest",
        ],
    );

    let re = Regex::new("\"browser_download_url\":.\"(?P<url>.*tar.gz)\"").unwrap();

    s = re.captures(&s).unwrap().name("url").unwrap().as_str().to_string();

    shell_command("wget", vec![&s]);

    shell_command("tar", vec!["-xf", "oxido*.tar.gz"]);

    shell_command("sudo", vec!["install", "oxido /usr/local/bin/"]);
}