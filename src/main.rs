mod installers;
mod uninstallers;

use uninstallers::{linux::uninstall_l, macos::uninstall_m, windows::uninstall_w};

use crate::installers::{linux::install_l, macos::install_m, windows::install_w};
use std::{
    env::{args, consts::OS},
    process::Command,
};

const HELP_MESSAGE: &str = "
Oxup is a tool for managing installations and packages of oxido.

Usage:
\toxup <command> [OPTIONS]

Commands:
\tadd\t\tadd packages to your project
\tinstall\t\tinstall oxido interpreter
\tremove\t\tremove packages from your project
\tupdate\t\tupdate the oxido interpreter to the latest version permitted by semver
\tuninstall\tuninstall oxido interpreter
\tversion\t\tprints the version

Options:
\t-W\tforce run as windows 
\t-L\tforce run as linux 
\t-M\tforce run as macos 
";

const VERSION: &str = "v1.0.0";

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        println!("oxup: missing overand\n{}", HELP_MESSAGE);
        std::process::exit(1)
    }

    let command = args[1].as_str();

    match command {
        "help" => println!("{}", HELP_MESSAGE),
        "install" | "update" => {
            let mut os = OS;
            if args.contains(&String::from("-L")) {
                os = "linux";
            } else if args.contains(&String::from("-M")) {
                os = "macos";
            } else if args.contains(&String::from("-W")) {
                os = "windows";
            }
            match os {
                "windows" => install_w(),
                "linux" => install_l(),
                "macos" => install_m(),
                _ => install_l(),
            }
        }
        "uninstall" => {
            let mut os = OS;
            if args.contains(&String::from("-L")) {
                os = "linux";
            } else if args.contains(&String::from("-M")) {
                os = "macos";
            } else if args.contains(&String::from("-W")) {
                os = "windows";
            }
            match os {
                "windows" => uninstall_w(),
                "linux" => uninstall_l(),
                "macos" => uninstall_m(),
                _ => uninstall_l(),
            }
        }
        "version" => println!("{}", VERSION),
        _ => {
            println!("oxup: missing overand\n{}", HELP_MESSAGE);
            std::process::exit(1)
        }
    }
}

fn shell_command(name: &str, args: Vec<&str>) -> String {
    String::from_utf8(Command::new(name).args(args).output().unwrap().stdout).unwrap()
}
