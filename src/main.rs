use crate::os::OS;
use clap::{command, Parser, Subcommand, ValueEnum};

mod init;
mod install;
mod os;
mod setup;
mod uninstall;
mod update;

/// Oxate is a tool for managing installations and packages of oxido.
#[derive(Parser, Debug)]
#[clap(author, version, about = "Oxate is a tool for managing installations and packages of oxido.", long_about = None)]
struct Oxate {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Install latest version of oxido
    #[command()]
    Install,

    /// Create a new oxate project
    #[command(arg_required_else_help = true)]
    Init { name: String },

    /// Setup oxate and its directories
    #[command()]
    Setup,
    /// Uninstall oxido
    #[command()]
    Uninstall,

    /// Update oxido to latest version
    #[command(arg_required_else_help = true)]
    Update { update: Updateable },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Updateable {
    Oxido,
    Oxate,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Oxate::parse();

    let os = OS::from();

    match args.command {
        Commands::Install => {
            install::install(os, false).await?;
        }
        Commands::Init { name } => init::init(name),
        Commands::Uninstall => {
            uninstall::uninstall(os);
        }
        Commands::Setup => setup::setup(os),
        Commands::Update { update } => update::update(update, os).await?,
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
            std::process::exit(1);
        };
    }

    #[macro_export]
    macro_rules! success {
        ($message:expr) => {
            println!("{} {}", "\x1b[32m=>\x1b[0m", $message)
        };
    }
}
