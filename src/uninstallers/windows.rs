use crate::{shell, success};
use colored::Colorize;

pub fn uninstall_w() {
    shell!("rm", vec![r"C:\bin\oxido"]);

    success!["Oxido has been uninstalled"];
}
