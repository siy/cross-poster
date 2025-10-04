use anyhow::Result;
use gray_matter::Matter;
use serde::{Deserialize, Serialize};

use crate::models::Article;

/// Frontmatter metadata extracted from markdown
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Frontmatter {
    /// Article title
    pub title: Option<String>,

    /// Article tags
    #[serde(default)]
    pub tags: Vec<String>,

    /// Canonical URL
    pub canonical_url: Option<String>,

    /// Publication status
    #[serde(default = "default_published")]
    pub published: bool,

    /// Cover image URL
    pub cover_image: Option<String>,

    /// Article description
    pub description: Option<String>,
}

fn default_published() -> bool {
    true
}

/// Parse markdown file with frontmatter
pub fn parse_markdown(content: &str) -> Result<Article> {
    let matter = Matter::<gray_matter::engine::YAML>::new();
    let result = matter
        .parse_with_struct::<Frontmatter>(content)
        .ok_or_else(|| anyhow::anyhow!("Failed to parse frontmatter"))?;

    let frontmatter = result.data;
    let body = result.content;

    // Require title from frontmatter or fail
    let title = frontmatter
        .title
        .ok_or_else(|| anyhow::anyhow!("Missing required 'title' field in frontmatter"))?;

    let mut article = Article::new(title, body).with_tags(frontmatter.tags);

    if let Some(canonical_url) = frontmatter.canonical_url {
        article = article.with_canonical_url(canonical_url);
    }

    article = article.with_published(frontmatter.published);

    if let Some(cover_image) = frontmatter.cover_image {
        article = article.with_cover_image(cover_image);
    }

    if let Some(description) = frontmatter.description {
        article = article.with_description(description);
    }

    Ok(article)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_markdown_with_yaml_frontmatter() {
        let content = r#"---
title: Test Article
tags: [rust, testing]
canonical_url: https://example.com/article
published: true
---

# Test Content

This is the article body."#;

        let article = parse_markdown(content).unwrap();
        assert_eq!(article.title, "Test Article");
        assert_eq!(article.tags, vec!["rust", "testing"]);
        assert_eq!(
            article.canonical_url,
            Some("https://example.com/article".to_string())
        );
        assert!(article.published);
        assert!(article.content.contains("# Test Content"));
    }

    #[test]
    fn test_parse_markdown_missing_title() {
        let content = r#"---
tags: [rust]
---

# Content"#;

        let result = parse_markdown(content);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Missing required 'title'"));
    }

    #[test]
    fn test_parse_markdown_no_frontmatter() {
        let content = "# Just Content\n\nNo frontmatter here.";

        let result = parse_markdown(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_markdown_with_optional_fields() {
        let content = r#"---
title: Full Article
tags: [rust, web]
canonical_url: https://example.com
cover_image: https://example.com/image.jpg
description: A test description
published: false
---

Content here."#;

        let article = parse_markdown(content).unwrap();
        assert_eq!(article.title, "Full Article");
        assert_eq!(
            article.cover_image,
            Some("https://example.com/image.jpg".to_string())
        );
        assert_eq!(article.description, Some("A test description".to_string()));
        assert!(!article.published);
    }
}
