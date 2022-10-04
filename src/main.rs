mod init;
mod install;
mod os;
mod setup;
mod uninstall;

use crate::os::OS;
use clap::{command, Parser, Subcommand};

/// Oxup is a tool for managing installations and packages of oxido.
#[derive(Parser, Debug)]
#[clap(author, version, about = "Oxup is a tool for managing installations and packages of oxido.", long_about = None)]
struct Oxup {
    /// Whether to output debug information
    #[clap(short, long, value_parser)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,

    /// Force run as windows
    #[clap(short, long, value_parser)]
    windows: bool,

    /// Force run as Macos
    #[clap(short, long, value_parser)]
    macos: bool,

    /// Force run as Linux
    #[clap(short, long, value_parser)]
    linux: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Install latest version of oxido
    #[command()]
    Install,

    /// Initialize a new project
    #[command(arg_required_else_help = true)]
    Init { file: String },

    /// Setup oxup and its directories
    #[command()]
    Setup,

    /// Uninstall oxido
    #[command()]
    Uninstall,

    /// Update oxido to latest version avaliable
    #[command()]
    Update,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Oxup::parse();

    let mut os = OS::from();
    if args.linux {
        os = OS::Linux;
    } else if args.macos {
        os = OS::Mac;
    } else if args.windows {
        os = OS::Windows;
    }

    match args.command {
        Commands::Install | Commands::Update => {
            install::install(os).await?;
        }
        Commands::Init { file } => init::init(file),
        Commands::Uninstall => {
            uninstall::uninstall(os);
        }
        Commands::Setup => setup::setup(os),
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
