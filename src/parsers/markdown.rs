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

/// Extract the first H1 heading from markdown content
fn extract_first_h1(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") {
            let title = trimmed.trim_start_matches("# ").trim();
            if !title.is_empty() {
                return Some(title.to_string());
            }
        }
    }
    None
}

/// Parse markdown file with frontmatter
pub fn parse_markdown(content: &str) -> Result<Article> {
    let matter = Matter::<gray_matter::engine::YAML>::new();
    let result = matter
        .parse_with_struct::<Frontmatter>(content)
        .ok_or_else(|| anyhow::anyhow!("Failed to parse frontmatter"))?;

    let frontmatter = result.data;
    let body = result.content;

    // Try to extract H1 from content
    let h1_title = extract_first_h1(&body);

    // Determine title based on frontmatter and H1
    let title = match (frontmatter.title, h1_title) {
        (Some(fm_title), Some(h1_title)) => {
            // Both present - they must match
            if fm_title.trim() != h1_title.trim() {
                anyhow::bail!(
                    "Title mismatch: frontmatter has '{}' but content starts with '# {}'. \
                    Please update in one place only to avoid inconsistency.",
                    fm_title, h1_title
                );
            }
            fm_title
        }
        (Some(fm_title), None) => {
            // Only frontmatter title
            fm_title
        }
        (None, Some(h1_title)) => {
            // Only H1 title
            h1_title
        }
        (None, None) => {
            // Neither - fail
            anyhow::bail!(
                "Missing required 'title'. Please provide either:\n\
                1. A 'title' field in the frontmatter, or\n\
                2. An H1 heading (# Title) at the start of your content"
            );
        }
    };

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

This is the article body with content."#;

        let article = parse_markdown(content).unwrap();
        assert_eq!(article.title, "Test Article");
        assert_eq!(article.tags, vec!["rust", "testing"]);
        assert_eq!(
            article.canonical_url,
            Some("https://example.com/article".to_string())
        );
        assert!(article.published);
        assert!(article.content.contains("This is the article body"));
    }

    #[test]
    fn test_parse_markdown_title_from_h1_only() {
        let content = r#"---
tags: [rust]
---

# Extracted Title

Content here."#;

        let article = parse_markdown(content).unwrap();
        assert_eq!(article.title, "Extracted Title");
        assert_eq!(article.tags, vec!["rust"]);
    }

    #[test]
    fn test_parse_markdown_missing_both_title_and_h1() {
        let content = r#"---
tags: [rust]
---

Just content without title or H1."#;

        let result = parse_markdown(content);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Missing required 'title'"));
    }

    #[test]
    fn test_parse_markdown_title_mismatch() {
        let content = r#"---
title: Frontmatter Title
tags: [rust]
---

# Different H1 Title

Content here."#;

        let result = parse_markdown(content);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Title mismatch"));
        assert!(err_msg.contains("Frontmatter Title"));
        assert!(err_msg.contains("Different H1 Title"));
    }

    #[test]
    fn test_parse_markdown_title_match() {
        let content = r#"---
title: Same Title
tags: [rust]
---

# Same Title

Content here."#;

        let article = parse_markdown(content).unwrap();
        assert_eq!(article.title, "Same Title");
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

    #[test]
    fn test_parse_title_with_colon_unquoted_fails() {
        // This test documents that unquoted values with colons fail to parse
        let content = r#"---
title: Java Backend Coding Technology: Writing Code in the Era of AI
tags: [AI, Java]
---

Content here."#;

        let result = parse_markdown(content);
        assert!(result.is_err(), "YAML requires quotes around values containing colons");
    }

    #[test]
    fn test_parse_title_with_colon_quoted() {
        let content = r#"---
title: "Java Backend Coding Technology: Writing Code in the Era of AI"
tags: [AI, Java, coding-technology]
published: true
description: "Revolutionary technology for writing deterministic, AI-friendly, high quality Java backend code."
---

## Introduction

Content here."#;

        let result = parse_markdown(content).unwrap();
        assert_eq!(result.title, "Java Backend Coding Technology: Writing Code in the Era of AI");
        assert_eq!(result.tags, vec!["AI", "Java", "coding-technology"]);
    }
}
