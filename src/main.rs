mod config;
mod platform;
mod profiles;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "journey-terminal",
    version,
    about = "Terminal profile & ricing manager (CLI-first)"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Show detected platform and config paths
    Doctor,

    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand, Debug)]
enum ConfigAction {
    /// Initialize configuration
    Init,

    /// Show current configuration
    Show,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Doctor => {
            println!("Platform: {}", platform::detect_platform());
            println!("Config dir: {}", config::config_dir().display());
            println!("Config file: {}", config::config_file_path().display());
        }

        Commands::Config { action } => match action {
            ConfigAction::Init => {
                if let Err(e) = config::init_config() {
                    eprintln!("Error initializing config: {}", e);
                }
            }

            ConfigAction::Show => match config::load_config() {
                Ok(cfg) => {
                    let path = config::config_file_path();
                    println!("{}", cfg.to_pretty_text(&path));
                }
                Err(e) => eprintln!("Error loading config: {}", e),
            },
        },
    }
}
