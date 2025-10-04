mod cli;
mod models;
mod parsers;
mod platforms;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, Commands, Config, ConfigAction, Platform};
use models::Article;
use parsers::{clean_ai_artifacts, fetch_from_devto_url, parse_devto_url, parse_markdown};
use platforms::{DevToClient, MediumClient};
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Config { action } => handle_config_command(action),
        Commands::Post {
            input,
            platforms,
            clean_ai,
            tags,
            canonical,
            dry_run,
        } => handle_post_command(input, platforms, clean_ai, tags, canonical, dry_run).await,
        Commands::Preview { input, clean_ai } => handle_preview_command(input, clean_ai).await,
    }
}

/// Handle configuration management commands
fn handle_config_command(action: ConfigAction) -> Result<()> {
    match action {
        ConfigAction::Init => Config::init(),
        ConfigAction::Show => Config::show(),
        ConfigAction::Path => Config::show_path(),
    }
}

/// Handle preview command - show processed content without posting
async fn handle_preview_command(input: String, clean_ai: bool) -> Result<()> {
    println!("Loading article from: {}", input);

    let mut article = load_article(&input).await?;

    if clean_ai {
        println!("Applying AI artifact cleaning...");
        article.content = clean_ai_artifacts(&article.content);
    }

    println!("\n--- PREVIEW ---\n");
    println!("Title: {}", article.title);
    if !article.tags.is_empty() {
        println!("Tags: {}", article.tags.join(", "));
    }
    if let Some(ref canonical) = article.canonical_url {
        println!("Canonical URL: {}", canonical);
    }
    if let Some(ref cover) = article.cover_image {
        println!("Cover Image: {}", cover);
    }
    if let Some(ref desc) = article.description {
        println!("Description: {}", desc);
    }
    println!("Published: {}", article.published);
    println!("\n--- CONTENT ---\n");
    println!("{}", article.content);
    println!("\n--- END PREVIEW ---");

    Ok(())
}

/// Handle post command - publish article to platforms
async fn handle_post_command(
    input: String,
    platforms: Vec<Platform>,
    clean_ai: bool,
    tags_override: Option<Vec<String>>,
    canonical_override: Option<String>,
    dry_run: bool,
) -> Result<()> {
    println!("Loading article from: {}", input);

    let mut article = load_article(&input).await?;

    // Apply AI cleaning if requested
    if clean_ai {
        println!("Applying AI artifact cleaning...");
        article.content = clean_ai_artifacts(&article.content);
    }

    // Apply overrides
    if let Some(tags) = tags_override {
        article.tags = tags;
    }
    if let Some(canonical) = canonical_override {
        article.canonical_url = Some(canonical);
    }

    if dry_run {
        println!("\n--- DRY RUN MODE ---");
        println!(
            "Would post to platforms: {}",
            platforms
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
        println!("\nArticle details:");
        println!("  Title: {}", article.title);
        println!("  Tags: {}", article.tags.join(", "));
        if let Some(ref canonical) = article.canonical_url {
            println!("  Canonical URL: {}", canonical);
        }
        println!("  Published: {}", article.published);
        println!("  Content length: {} characters", article.content.len());
        println!("\n--- DRY RUN COMPLETE (no actual posting) ---");
        return Ok(());
    }

    // Load config for API credentials
    let config = Config::load().context("Failed to load config. Run 'config init' first.")?;

    println!("\nPublishing to {} platform(s)...\n", platforms.len());

    let mut results = Vec::new();

    for platform in platforms {
        print!("Publishing to {}... ", platform);

        let result = match platform {
            Platform::DevTo => {
                let client = DevToClient::new(config.dev_to.api_key.clone());
                publish_to_devto(&client, &article).await
            }
            Platform::Medium => {
                let client = MediumClient::new(config.medium.access_token.clone());
                publish_to_medium(&client, &article).await
            }
        };

        match result {
            Ok(url) => {
                println!("✓ Success");
                results.push((platform, Ok(url)));
            }
            Err(e) => {
                println!("✗ Failed");
                results.push((platform, Err(e)));
            }
        }
    }

    // Display summary
    println!("\n--- RESULTS ---");
    for (platform, result) in results {
        match result {
            Ok(url) => {
                println!("✓ {}: {}", platform, url);
            }
            Err(e) => {
                println!("✗ {}: {}", platform, e);
            }
        }
    }

    Ok(())
}

/// Load article from file or dev.to URL
async fn load_article(input: &str) -> Result<Article> {
    // Check if input is a dev.to URL
    if parse_devto_url(input).is_ok() {
        // Fetch from dev.to - need API key from config
        let config = Config::load().context("Failed to load config. Run 'config init' first.")?;

        fetch_from_devto_url(input, &config.dev_to.api_key)
            .await
            .context("Failed to fetch article from dev.to URL")
    } else {
        // Assume it's a file path - validate and canonicalize to prevent path traversal
        let path = Path::new(input);

        // Canonicalize the path to resolve .. and symlinks
        let canonical_path = path
            .canonicalize()
            .context(format!("Invalid or inaccessible file path: {}", input))?;

        // Verify it's a file (not a directory or special file)
        if !canonical_path.is_file() {
            anyhow::bail!("Path is not a regular file: {}", input);
        }

        let content = fs::read_to_string(&canonical_path).context(format!(
            "Failed to read markdown file: {}",
            canonical_path.display()
        ))?;

        parse_markdown(&content).context("Failed to parse markdown file")
    }
}

/// Publish article to dev.to
async fn publish_to_devto(client: &DevToClient, article: &Article) -> Result<String> {
    client
        .publish_article(article)
        .await
        .context("Failed to publish to dev.to")
}

/// Publish article to Medium
async fn publish_to_medium(client: &MediumClient, article: &Article) -> Result<String> {
    client
        .publish_article(article)
        .await
        .context("Failed to publish to Medium")
}
