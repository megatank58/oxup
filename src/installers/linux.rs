use regex::Regex;
use crate::{shell_command, current_shell};

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

    shell_command("mkdir", vec!["$HOME/.oxido"]);
    shell_command("mv", vec!["oxido* $HOME/.oxido"]);
    shell_command("export", vec!["PATH:$HOME/.oxido"]);

    if current_shell() == "bash" {
        shell_command("echo", vec!["\"export PATH:$HOME/.oxido\" >> $HOME/.bashrc"]);
    } else if current_shell() == "zsh" {
        shell_command("echo", vec!["\"export PATH:$HOME/.oxido\" >> $HOME/.zshrc"]);
    } 
    
}