mod init;
mod install;
mod os;
mod setup;
mod uninstall;
mod update;

use crate::os::OS;
use clap::{command, Parser, Subcommand, ValueEnum};

/// Oxup is a tool for managing installations and packages of oxido.
#[derive(Parser, Debug)]
#[clap(author, version, about = "Oxup is a tool for managing installations and packages of oxido.", long_about = None)]
struct Oxup {
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

    /// Do not check for updates
    #[clap(short, long, value_parser)]
    no_update: bool,
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
    #[command(arg_required_else_help = true)]
    Update { update: Updateable },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Updateable {
    Oxido,
    Oxup,
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
        Commands::Install => {
            install::install(os, false).await?;
        }
        Commands::Init { file } => init::init(file),
        Commands::Uninstall => {
            uninstall::uninstall(os);
        }
        Commands::Setup => setup::setup(os),
        Commands::Update { update } => update::update(update, os).await?,
    }

    let current_version = env!("CARGO_PKG_VERSION");

    let new_version = reqwest::get("https://raw.githubusercontent.com/oxidic/oxup/main/.version")
        .await?
        .text()
        .await?;

    if !args.no_update && current_version != new_version {
        info![format!("Oxup v{new_version} is avaliable for download.")];
    }

    Ok(())
}

#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! info {
        ($message:expr) => {
            println!("{} {}", "\x1b[1minfo:\x1b[0m", $message)
        };
    }

    #[macro_export]
    macro_rules! error {
        ($message:expr) => {
            println!("{} {}", "\x1b[1m\x1b[31merror:\x1b[0m", $message)
        };
    }

    #[macro_export]
    macro_rules! success {
        ($message:expr) => {
            println!("{} {}", "\x1b[32m=>\x1b[0m", $message)
        };
    }
}
