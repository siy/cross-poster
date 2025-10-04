use clap::{Parser, Subcommand};

/// Cross-post articles to dev.to and Medium
#[derive(Parser, Debug)]
#[command(name = "article-cross-poster")]
#[command(about = "Cross-post articles to dev.to and Medium", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available commands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Post an article to one or more platforms
    Post {
        /// Path to markdown file or dev.to URL
        input: String,

        /// Target platforms (comma-separated: devto,medium)
        #[arg(short = 't', long = "to", value_delimiter = ',', required = true)]
        platforms: Vec<Platform>,

        /// Apply AI artifact cleaning to content
        #[arg(long)]
        clean_ai: bool,

        /// Override tags from frontmatter (comma-separated)
        #[arg(long, value_delimiter = ',')]
        tags: Option<Vec<String>>,

        /// Set canonical URL
        #[arg(long)]
        canonical: Option<String>,

        /// Dry run - show what would be posted without actually posting
        #[arg(long)]
        dry_run: bool,
    },

    /// Preview processed content without posting
    Preview {
        /// Path to markdown file or dev.to URL
        input: String,

        /// Apply AI artifact cleaning to content
        #[arg(long)]
        clean_ai: bool,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

/// Configuration management actions
#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Initialize config file with template
    Init,

    /// Show current configuration (with masked credentials)
    Show,

    /// Show config file path
    Path,
}

/// Supported platforms
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Platform {
    DevTo,
    Medium,
}

impl std::str::FromStr for Platform {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "devto" | "dev.to" => Ok(Platform::DevTo),
            "medium" => Ok(Platform::Medium),
            _ => Err(format!("Unknown platform: '{}'. Valid options: devto, medium", s)),
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::DevTo => write!(f, "dev.to"),
            Platform::Medium => write!(f, "Medium"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_from_str() {
        assert_eq!("devto".parse::<Platform>().unwrap(), Platform::DevTo);
        assert_eq!("dev.to".parse::<Platform>().unwrap(), Platform::DevTo);
        assert_eq!("medium".parse::<Platform>().unwrap(), Platform::Medium);
        assert_eq!("MEDIUM".parse::<Platform>().unwrap(), Platform::Medium);
        assert!("invalid".parse::<Platform>().is_err());
    }

    #[test]
    fn test_platform_display() {
        assert_eq!(Platform::DevTo.to_string(), "dev.to");
        assert_eq!(Platform::Medium.to_string(), "Medium");
    }
}
