use regex::Regex;

use crate::shell_command;

pub fn install_w() {
    let mut s = shell_command(
        "curl",
        vec!["https://api.github.com/repositories/500013933/releases/latest"],
    );

    let re = Regex::new("\"browser_download_url\":.\"(?P<url>.*gnu.zip)\"").unwrap();

    s = re
        .captures(&s)
        .unwrap()
        .name("url")
        .unwrap()
        .as_str()
        .to_string();

    shell_command("wget", vec![&s]);

    shell_command("unzip", vec!["oxido-windows.gnu.zip"]);

    shell_command("mkdir", vec![r"C:\bin"]);

    shell_command("move", vec![r"oxido.exe C:\oxido"]);

    shell_command("setx", vec!["PATH \"C:\\oxido;%PATH%\""]);
}
