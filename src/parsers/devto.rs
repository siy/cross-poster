use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::models::Article;
use crate::platforms::devto::DevToClient;

/// Regex to extract dev.to article ID from URL
/// Matches URLs like:
/// - https://dev.to/username/article-slug-123
/// - https://dev.to/username/article-slug-123abc
static DEVTO_URL_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"https?://dev\.to/[^/]+/[^/]+-([a-z0-9]+)/?$").unwrap());

/// Parse dev.to URL and extract article ID
pub fn parse_devto_url(url: &str) -> Result<String> {
    let captures = DEVTO_URL_PATTERN
        .captures(url)
        .context("Invalid dev.to URL format - expected https://dev.to/username/article-slug-id")?;

    let article_id = captures
        .get(1)
        .context("Could not extract article ID from dev.to URL")?
        .as_str()
        .to_string();

    Ok(article_id)
}

/// Fetch article from dev.to URL
pub async fn fetch_from_devto_url(url: &str, api_key: &str) -> Result<Article> {
    let article_id = parse_devto_url(url)?;

    let client = DevToClient::new(api_key.to_string());
    let article = client
        .fetch_article(&article_id)
        .await
        .context("Failed to fetch article from dev.to")?;

    Ok(article)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_devto_url_valid() {
        let url = "https://dev.to/username/my-awesome-article-1a2b3c";
        let result = parse_devto_url(url);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1a2b3c");
    }

    #[test]
    fn test_parse_devto_url_with_trailing_slash() {
        let url = "https://dev.to/username/my-awesome-article-1a2b3c/";
        let result = parse_devto_url(url);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1a2b3c");
    }

    #[test]
    fn test_parse_devto_url_invalid() {
        let url = "https://medium.com/@user/article";
        let result = parse_devto_url(url);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_devto_url_missing_id() {
        let url = "https://dev.to/username/";
        let result = parse_devto_url(url);
        assert!(result.is_err());
    }
}
