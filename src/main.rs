mod init;
mod install;
mod os;
mod setup;
mod uninstall;

use crate::os::OS;
use std::env::args;

const MESSAGE: &str = "
Oxup is a tool for managing installations and packages of oxido.

Usage:
\toxup <command> [OPTIONS]

Commands:
\tadd\t\tAdd packages to your project
\tinstall\t\tInstall oxido interpreter
\tinit\t\tInitialize a new >
\tsetup\t\tSetup oxup directories
\tremove\t\tRemove packages from your project
\tupdate\t\tUpdate the oxido interpreter to the latest version permitted by semver
\tuninstall\tUninstall oxido interpreter
\tversion\t\tPrints the version

Options:
\t-W\tforce run as windows 
\t-L\tforce run as linux 
\t-M\tforce run as macos 
";

const VERSION: &str = "v1.0.0";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        println!("{MESSAGE}");
        std::process::exit(1)
    }

    let command = args[1].as_str();
    let mut os = OS::from();
    if args.contains(&String::from("-L")) {
        os = OS::Linux;
    } else if args.contains(&String::from("-M")) {
        os = OS::Mac;
    } else if args.contains(&String::from("-W")) {
        os = OS::Windows;
    }

    match command {
        "help" => println!("{MESSAGE}"),
        "install" | "update" => {
            install::install(os).await?;
        }
        "init" => init::init(),
        "uninstall" => {
            uninstall::uninstall(os);
        }
        "setup" => setup::setup(os),
        "version" => println!("{VERSION}"),
        _ => {
            error![format!(
                "command `{command}` not found!\nrun --help to view all commands"
            )];
            std::process::exit(1)
        }
    }

    Ok(())
}

#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! info {
        ($message:expr) => {
            println!("{} {}", "\x1b[1minfo:\x1b[0m", $message);
        };
    }

    #[macro_export]
    macro_rules! error {
        ($message:expr) => {
            println!("{} {}", "\x1b[1m\x1b[31merror:\x1b[0m", $message);
        };
    }

    #[macro_export]
    macro_rules! success {
        ($message:expr) => {
            println!("{} {}", "\x1b[32m=>\x1b[0m", $message)
        };
    }
}
