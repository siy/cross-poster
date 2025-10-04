mod cli;
mod models;
mod parsers;
mod platforms;

use anyhow::Result;
use clap::{Parser, Subcommand};
use cli::Config;

#[derive(Parser)]
#[command(name = "article-cross-poster")]
#[command(about = "Cross-post articles to dev.to and Medium", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize configuration file
    #[command(name = "config")]
    Config {
        #[command(subcommand)]
        config_command: ConfigCommands,
    },
    /// Post an article to platforms
    #[command(name = "post")]
    Post {
        /// Path to markdown file or dev.to URL
        source: String,

        /// Target platforms (comma-separated: devto,medium)
        #[arg(short, long, value_delimiter = ',')]
        platforms: Vec<String>,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Initialize config file with template
    Init,
    /// Show current configuration
    Show,
    /// Show config file path
    Path,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Config { config_command } => match config_command {
            ConfigCommands::Init => {
                Config::init()?;
            }
            ConfigCommands::Show => {
                Config::show()?;
            }
            ConfigCommands::Path => {
                Config::show_path()?;
            }
        },
        Commands::Post { source, platforms } => {
            println!("Posting from: {}", source);
            println!("To platforms: {:?}", platforms);
            println!("(Implementation pending)");
        }
    }

    Ok(())
}
