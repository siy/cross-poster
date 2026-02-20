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

        /// Content format for Medium (markdown or html)
        #[arg(long, default_value = "markdown")]
        format: ContentFormat,
    },

    /// Preview processed content without posting
    Preview {
        /// Path to markdown file or dev.to URL
        input: String,

        /// Apply AI artifact cleaning to content
        #[arg(long)]
        clean_ai: bool,
    },

    /// List published articles from a platform
    #[command(long_about = "List articles from a platform.\n\n\
        dev.to: Supports pagination and filtering by state.\n\
        Medium: Returns at most 10 recent articles via RSS. No pagination or state filtering.")]
    List {
        /// Platform to list from (devto or medium)
        #[arg(long = "from", required = true)]
        platform: Platform,

        /// Page number (dev.to only, default: 1)
        #[arg(long, default_value = "1")]
        page: u32,

        /// Articles per page (dev.to only, default: 30)
        #[arg(long, default_value = "30")]
        per_page: u32,

        /// Article state filter (dev.to only: published, unpublished, all)
        #[arg(long, default_value = "published")]
        state: ArticleState,
    },

    /// Fetch a single article by ID
    #[command(long_about = "Fetch a single article by ID.\n\n\
        Only dev.to is supported. Medium does not provide an article fetch API.")]
    Fetch {
        /// Article ID
        id: String,

        /// Platform to fetch from (only devto supported)
        #[arg(long = "from", required = true)]
        platform: Platform,
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

/// Article state filter for listing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArticleState {
    Published,
    Unpublished,
    All,
}

/// Content format for Medium posts
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentFormat {
    Markdown,
    Html,
}

impl std::str::FromStr for Platform {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "devto" | "dev.to" => Ok(Platform::DevTo),
            "medium" => Ok(Platform::Medium),
            _ => Err(format!(
                "Unknown platform: '{}'. Valid options: devto, medium",
                s
            )),
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

impl std::str::FromStr for ArticleState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "published" => Ok(ArticleState::Published),
            "unpublished" => Ok(ArticleState::Unpublished),
            "all" => Ok(ArticleState::All),
            _ => Err(format!(
                "Unknown state: '{}'. Valid options: published, unpublished, all",
                s
            )),
        }
    }
}

impl std::fmt::Display for ArticleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArticleState::Published => write!(f, "published"),
            ArticleState::Unpublished => write!(f, "unpublished"),
            ArticleState::All => write!(f, "all"),
        }
    }
}

impl std::str::FromStr for ContentFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "markdown" | "md" => Ok(ContentFormat::Markdown),
            "html" => Ok(ContentFormat::Html),
            _ => Err(format!(
                "Unknown format: '{}'. Valid options: markdown, html",
                s
            )),
        }
    }
}

impl std::fmt::Display for ContentFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentFormat::Markdown => write!(f, "markdown"),
            ContentFormat::Html => write!(f, "html"),
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

    #[test]
    fn test_content_format_from_str() {
        assert_eq!(
            "markdown".parse::<ContentFormat>().unwrap(),
            ContentFormat::Markdown
        );
        assert_eq!(
            "md".parse::<ContentFormat>().unwrap(),
            ContentFormat::Markdown
        );
        assert_eq!(
            "html".parse::<ContentFormat>().unwrap(),
            ContentFormat::Html
        );
        assert_eq!(
            "HTML".parse::<ContentFormat>().unwrap(),
            ContentFormat::Html
        );
        assert!("invalid".parse::<ContentFormat>().is_err());
    }

    #[test]
    fn test_content_format_display() {
        assert_eq!(ContentFormat::Markdown.to_string(), "markdown");
        assert_eq!(ContentFormat::Html.to_string(), "html");
    }

    #[test]
    fn test_article_state_from_str() {
        assert_eq!(
            "published".parse::<ArticleState>().unwrap(),
            ArticleState::Published
        );
        assert_eq!(
            "unpublished".parse::<ArticleState>().unwrap(),
            ArticleState::Unpublished
        );
        assert_eq!("all".parse::<ArticleState>().unwrap(), ArticleState::All);
        assert_eq!("ALL".parse::<ArticleState>().unwrap(), ArticleState::All);
        assert!("invalid".parse::<ArticleState>().is_err());
    }

    #[test]
    fn test_article_state_display() {
        assert_eq!(ArticleState::Published.to_string(), "published");
        assert_eq!(ArticleState::Unpublished.to_string(), "unpublished");
        assert_eq!(ArticleState::All.to_string(), "all");
    }
}
