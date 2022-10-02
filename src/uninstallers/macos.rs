use crate::success;
use colored::Colorize;

pub fn uninstall_m() {
    std::fs::remove_file(format!(
        "{}/.oxido/bin/oxido",
        std::env::var("HOME").unwrap()
    ))
    .unwrap();

    success!["Oxido has been uninstalled"];
}
