use anyhow::{bail, Result};
use regex::Regex;

use crate::models::Article;

/// Platform types for sanitization
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Platform {
    DevTo,
    Medium,
}

/// Sanitize article for specific platform
pub fn sanitize_for_platform(article: &mut Article, platform: Platform) -> Result<()> {
    match platform {
        Platform::DevTo => sanitize_for_devto(article)?,
        Platform::Medium => sanitize_for_medium(article)?,
    }
    Ok(())
}

/// Sanitize for dev.to platform
fn sanitize_for_devto(article: &mut Article) -> Result<()> {
    // Validate tag count (max 4 for dev.to)
    if article.tags.len() > 4 {
        bail!(
            "dev.to allows maximum 4 tags, found {}",
            article.tags.len()
        );
    }

    // Validate URLs in content
    validate_image_urls(&article.content)?;

    Ok(())
}

/// Sanitize for Medium platform
fn sanitize_for_medium(article: &mut Article) -> Result<()> {
    // Validate tag count (max 5 for Medium)
    if article.tags.len() > 5 {
        bail!(
            "Medium allows maximum 5 tags, found {}",
            article.tags.len()
        );
    }

    // Remove dev.to liquid tags ({% ... %})
    article.content = remove_liquid_tags(&article.content);

    // Validate URLs in content
    validate_image_urls(&article.content)?;

    Ok(())
}

/// Remove Liquid tags from content
fn remove_liquid_tags(content: &str) -> String {
    let liquid_tag_pattern = Regex::new(r"\{%.*?%\}").unwrap();
    liquid_tag_pattern.replace_all(content, "").to_string()
}

/// Validate image URLs in content
fn validate_image_urls(content: &str) -> Result<()> {
    let image_pattern = Regex::new(r"!\[.*?\]\((.*?)\)").unwrap();

    for cap in image_pattern.captures_iter(content) {
        if let Some(url) = cap.get(1) {
            let url_str = url.as_str();
            if !url_str.starts_with("http://") && !url_str.starts_with("https://") {
                bail!("Invalid image URL (must be absolute): {}", url_str);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_devto_tag_validation() {
        let mut article = Article::new("Test".to_string(), "Content".to_string())
            .with_tags(vec![
                "tag1".to_string(),
                "tag2".to_string(),
                "tag3".to_string(),
                "tag4".to_string(),
                "tag5".to_string(),
            ]);

        let result = sanitize_for_devto(&mut article);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("maximum 4 tags"));
    }

    #[test]
    fn test_devto_tag_validation_success() {
        let mut article = Article::new("Test".to_string(), "Content".to_string())
            .with_tags(vec![
                "tag1".to_string(),
                "tag2".to_string(),
                "tag3".to_string(),
            ]);

        let result = sanitize_for_devto(&mut article);
        assert!(result.is_ok());
    }

    #[test]
    fn test_medium_tag_validation() {
        let mut article = Article::new("Test".to_string(), "Content".to_string())
            .with_tags(vec![
                "tag1".to_string(),
                "tag2".to_string(),
                "tag3".to_string(),
                "tag4".to_string(),
                "tag5".to_string(),
                "tag6".to_string(),
            ]);

        let result = sanitize_for_medium(&mut article);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("maximum 5 tags"));
    }

    #[test]
    fn test_remove_liquid_tags() {
        let content = "Some content {% tweet 123456 %} more content {% github user/repo %}";
        let cleaned = remove_liquid_tags(content);
        assert_eq!(cleaned, "Some content  more content ");
    }

    #[test]
    fn test_validate_image_urls_valid() {
        let content = "![alt](https://example.com/image.jpg)";
        let result = validate_image_urls(content);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_image_urls_invalid() {
        let content = "![alt](relative/path/image.jpg)";
        let result = validate_image_urls(content);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("must be absolute"));
    }

    #[test]
    fn test_sanitize_for_medium_removes_liquid_tags() {
        let mut article = Article::new(
            "Test".to_string(),
            "Content {% tweet 123 %} here".to_string(),
        )
        .with_tags(vec!["tag1".to_string()]);

        sanitize_for_medium(&mut article).unwrap();
        assert_eq!(article.content, "Content  here");
    }
}
