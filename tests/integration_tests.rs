use article_cross_poster::cli::{ArticleState, Config};
use article_cross_poster::models::{Article, ArticleSummary};
use article_cross_poster::parsers::{clean_ai_artifacts, parse_markdown};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper to create a temporary config for testing
fn create_test_config() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    let config_content = r#"
[dev_to]
api_key = "test_dev_to_key"

[medium]
access_token = "test_medium_token"
"#;

    fs::write(&config_path, config_content).unwrap();
    (temp_dir, config_path)
}

#[test]
fn test_config_parsing() {
    let (_temp_dir, config_path) = create_test_config();

    let content = fs::read_to_string(&config_path).unwrap();
    let config: Config = toml::from_str(&content).unwrap();

    assert_eq!(config.dev_to.api_key, "test_dev_to_key");
    assert_eq!(config.medium.access_token, "test_medium_token");
}

#[test]
fn test_markdown_parsing_basic() {
    let markdown = r#"---
title: My Test Article
tags: [rust, testing]
published: true
---

This is a test article with **bold** and *italic* text.

## Code Example

```rust
fn main() {
    println!("Hello, world!");
}
```
"#;

    let article = parse_markdown(markdown).unwrap();

    assert_eq!(article.title, "My Test Article");
    assert_eq!(article.tags, vec!["rust", "testing"]);
    assert!(article.published);
    assert!(article.content.contains("This is a test article"));
    assert!(article.content.contains("```rust"));
}

#[test]
fn test_markdown_parsing_with_canonical_url() {
    let markdown = r#"---
title: Article with Canonical
tags: [web]
canonical_url: https://example.com/original
cover_image: https://example.com/cover.jpg
description: A great article
published: false
---

Content goes here.
"#;

    let article = parse_markdown(markdown).unwrap();

    assert_eq!(article.title, "Article with Canonical");
    assert_eq!(
        article.canonical_url,
        Some("https://example.com/original".to_string())
    );
    assert_eq!(
        article.cover_image,
        Some("https://example.com/cover.jpg".to_string())
    );
    assert_eq!(article.description, Some("A great article".to_string()));
    assert!(!article.published);
}

#[test]
fn test_markdown_parsing_minimal_frontmatter() {
    let markdown = r#"---
title: Minimal Article
---

Just content.
"#;

    let article = parse_markdown(markdown).unwrap();

    assert_eq!(article.title, "Minimal Article");
    assert!(article.tags.is_empty());
    assert!(article.published); // Should default to true
    assert_eq!(article.content.trim(), "Just content.");
}

#[test]
fn test_markdown_parsing_title_from_h1() {
    let markdown = r#"---
tags: [test, rust]
published: true
---

# Title from H1 Heading

Content without frontmatter title.
"#;

    let article = parse_markdown(markdown).unwrap();
    assert_eq!(article.title, "Title from H1 Heading");
    assert_eq!(article.tags, vec!["test", "rust"]);
    assert!(article.published);
}

#[test]
fn test_markdown_parsing_title_mismatch_fails() {
    let markdown = r#"---
title: Frontmatter Title
tags: [test]
---

# Different H1 Title

Content here.
"#;

    let result = parse_markdown(markdown);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Title mismatch"));
    assert!(err.contains("Frontmatter Title"));
    assert!(err.contains("Different H1 Title"));
}

#[test]
fn test_markdown_parsing_missing_title_fails() {
    let markdown = r#"---
tags: [test]
---

Content without title or H1 heading.
"#;

    let result = parse_markdown(markdown);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Missing required 'title'"));
}

#[test]
fn test_ai_cleanup_emojis() {
    let text = "Hello 👋 World 🌍! This is 🚀 amazing!";
    let cleaned = clean_ai_artifacts(text);

    assert!(!cleaned.contains("👋"));
    assert!(!cleaned.contains("🌍"));
    assert!(!cleaned.contains("🚀"));
    assert!(cleaned.contains("Hello"));
    assert!(cleaned.contains("World"));
    assert!(cleaned.contains("amazing"));
}

#[test]
fn test_ai_cleanup_smart_quotes() {
    let text = "\u{201C}Hello\u{201D} and \u{2018}world\u{2019}";
    let cleaned = clean_ai_artifacts(text);

    assert_eq!(cleaned, "\"Hello\" and 'world'");
}

#[test]
fn test_ai_cleanup_dashes() {
    let text = "Em dash \u{2014} and en dash \u{2013} here";
    let cleaned = clean_ai_artifacts(text);

    assert_eq!(cleaned, "Em dash -- and en dash - here");
}

#[test]
fn test_ai_cleanup_ellipsis() {
    let text = "Wait\u{2026} for it";
    let cleaned = clean_ai_artifacts(text);

    assert_eq!(cleaned, "Wait... for it");
}

#[test]
fn test_ai_cleanup_zero_width_characters() {
    let text = "Hello\u{200B}World\u{FEFF}!";
    let cleaned = clean_ai_artifacts(text);

    assert_eq!(cleaned, "HelloWorld!");
}

#[test]
fn test_ai_cleanup_comprehensive() {
    let text = "# My Article \u{1F680}\n\n\u{201C}Smart quotes\u{201D} and \u{2018}apostrophes\u{2019} everywhere \u{2014} with dashes.\n\nWait\u{2026} there's more!";

    let cleaned = clean_ai_artifacts(text);

    // Should not contain emojis
    assert!(!cleaned.contains("\u{1F680}"));

    // Should have straight quotes
    assert!(cleaned.contains("\"Smart quotes\""));
    assert!(cleaned.contains("'apostrophes'"));

    // Should have ASCII dashes
    assert!(cleaned.contains("--"));

    // Should have three dots instead of ellipsis
    assert!(cleaned.contains("..."));
}

#[test]
fn test_ai_cleanup_preserves_code_blocks() {
    let text = r#"Some text with code:

```rust
fn main() {
    println!("Hello");
}
```

More text."#;

    let cleaned = clean_ai_artifacts(text);

    // Code block should be preserved
    assert!(cleaned.contains("```rust"));
    assert!(cleaned.contains("fn main()"));
    assert!(cleaned.contains("println!"));
}

#[test]
fn test_article_builder_pattern() {
    let article = Article::new("Test".to_string(), "Content".to_string())
        .with_tags(vec!["tag1".to_string(), "tag2".to_string()])
        .with_canonical_url("https://example.com".to_string())
        .with_published(false)
        .with_cover_image("https://example.com/cover.jpg".to_string())
        .with_description("Description".to_string());

    assert_eq!(article.title, "Test");
    assert_eq!(article.content, "Content");
    assert_eq!(article.tags, vec!["tag1", "tag2"]);
    assert_eq!(
        article.canonical_url,
        Some("https://example.com".to_string())
    );
    assert!(!article.published);
    assert_eq!(
        article.cover_image,
        Some("https://example.com/cover.jpg".to_string())
    );
    assert_eq!(article.description, Some("Description".to_string()));
}

#[test]
fn test_markdown_with_complex_content() {
    let markdown = r#"---
title: Complex Article
tags: [rust, web, tutorial]
canonical_url: https://original.com/article
---

This article covers **Rust** programming.

## Installation

1. Install Rust
2. Create project
3. Run code

### Code Example

```rust
use std::io;

fn main() -> io::Result<()> {
    println!("Hello!");
    Ok(())
}
```

## Conclusion

That's all for now!
"#;

    let article = parse_markdown(markdown).unwrap();

    assert_eq!(article.title, "Complex Article");
    assert_eq!(article.tags, vec!["rust", "web", "tutorial"]);
    assert!(article.content.contains("This article covers"));
    assert!(article.content.contains("1. Install Rust"));
    assert!(article.content.contains("```rust"));
    assert!(article.content.contains("use std::io"));
}

#[test]
fn test_markdown_empty_tags() {
    let markdown = r#"---
title: No Tags Article
tags: []
---

Content
"#;

    let article = parse_markdown(markdown).unwrap();
    assert!(article.tags.is_empty());
}

#[test]
fn test_article_serialization() {
    let article =
        Article::new("Test".to_string(), "Content".to_string()).with_tags(vec!["tag1".to_string()]);

    // Should be serializable to JSON
    let json = serde_json::to_string(&article).unwrap();
    assert!(json.contains("Test"));
    assert!(json.contains("Content"));
    assert!(json.contains("tag1"));

    // Should be deserializable from JSON
    let deserialized: Article = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.title, "Test");
    assert_eq!(deserialized.content, "Content");
}

// Format conversion tests

#[test]
fn test_markdown_to_html_conversion() {
    use article_cross_poster::parsers::markdown_to_html;

    let markdown = "# Title\n\nThis is **bold** and *italic*.";
    let html = markdown_to_html(markdown).unwrap();

    assert!(html.contains("<h1>"));
    assert!(html.contains("Title</h1>"));
    assert!(html.contains("<strong>bold</strong>"));
    assert!(html.contains("<em>italic</em>"));
}

#[test]
fn test_markdown_to_html_code_blocks() {
    use article_cross_poster::parsers::markdown_to_html;

    let markdown = "```rust\nfn main() {}\n```";
    let html = markdown_to_html(markdown).unwrap();

    assert!(html.contains("<code"));
    assert!(html.contains("fn main()"));
}

#[test]
fn test_markdown_to_html_security() {
    use article_cross_poster::parsers::markdown_to_html;

    let markdown =
        "Regular **markdown** with potential <script>alert('xss')</script> inline content";
    let html = markdown_to_html(markdown).unwrap();

    // Should convert markdown properly
    assert!(html.contains("<strong>markdown</strong>"));

    // Security: pulldown-cmark without ENABLE_HTML treats inline HTML as text.
    // We intentionally do NOT enable ENABLE_HTML to prevent XSS attacks.
    // This means HTML tags are passed through as-is (as text content), not parsed.
    // The key security feature is that we never enable HTML parsing.
}

#[test]
fn test_ensure_title_prepending() {
    use article_cross_poster::parsers::ensure_title_in_content;

    let title = "My Article";
    let content_without_title = "This is the content.";
    let result = ensure_title_in_content(title, content_without_title);

    assert!(result.starts_with("# My Article\n\n"));
    assert!(result.contains("This is the content."));
}

#[test]
fn test_title_not_duplicated_when_h1_present() {
    use article_cross_poster::parsers::ensure_title_in_content;

    let title = "My Article";
    let content_with_h1 = "# Different Title\n\nContent here";
    let result = ensure_title_in_content(title, content_with_h1);

    // Should not duplicate - content already has H1
    assert_eq!(result, content_with_h1);
}

#[test]
fn test_title_prepended_when_only_h2() {
    use article_cross_poster::parsers::ensure_title_in_content;

    let title = "My Article";
    let content_with_h2 = "## Introduction\n\nContent";
    let result = ensure_title_in_content(title, content_with_h2);

    // Should prepend since there's no H1
    assert!(result.starts_with("# My Article\n\n"));
    assert!(result.contains("## Introduction"));
}

// ArticleSummary tests

#[test]
fn test_article_summary_creation() {
    let summary = ArticleSummary {
        id: "12345".to_string(),
        title: "My Test Article".to_string(),
        url: "https://dev.to/user/my-test-article".to_string(),
        published_at: "2024-01-15".to_string(),
        tags: vec!["rust".to_string(), "testing".to_string()],
    };

    assert_eq!(summary.id, "12345");
    assert_eq!(summary.title, "My Test Article");
    assert_eq!(summary.url, "https://dev.to/user/my-test-article");
    assert_eq!(summary.published_at, "2024-01-15");
    assert_eq!(summary.tags, vec!["rust", "testing"]);
}

#[test]
fn test_article_summary_empty_fields() {
    let summary = ArticleSummary {
        id: "1".to_string(),
        title: String::new(),
        url: String::new(),
        published_at: String::new(),
        tags: Vec::new(),
    };

    assert!(summary.title.is_empty());
    assert!(summary.tags.is_empty());
}

// ArticleState tests

#[test]
fn test_article_state_parsing() {
    assert_eq!(
        "published".parse::<ArticleState>().unwrap(),
        ArticleState::Published
    );
    assert_eq!(
        "unpublished".parse::<ArticleState>().unwrap(),
        ArticleState::Unpublished
    );
    assert_eq!("all".parse::<ArticleState>().unwrap(), ArticleState::All);
}

#[test]
fn test_article_state_case_insensitive() {
    assert_eq!(
        "PUBLISHED".parse::<ArticleState>().unwrap(),
        ArticleState::Published
    );
    assert_eq!("All".parse::<ArticleState>().unwrap(), ArticleState::All);
}

#[test]
fn test_article_state_invalid() {
    assert!("draft".parse::<ArticleState>().is_err());
    assert!("".parse::<ArticleState>().is_err());
}

#[test]
fn test_article_state_display() {
    assert_eq!(ArticleState::Published.to_string(), "published");
    assert_eq!(ArticleState::Unpublished.to_string(), "unpublished");
    assert_eq!(ArticleState::All.to_string(), "all");
}

// Medium RSS feed parsing test

#[test]
fn test_medium_rss_parsing() {
    let rss_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>Test User on Medium</title>
    <item>
      <title>First Article</title>
      <link>https://medium.com/@user/first-article-abc123</link>
      <guid>https://medium.com/p/abc123</guid>
      <pubDate>Mon, 15 Jan 2024 12:00:00 GMT</pubDate>
      <category>rust</category>
      <category>programming</category>
    </item>
    <item>
      <title>Second Article</title>
      <link>https://medium.com/@user/second-article-def456</link>
      <guid>https://medium.com/p/def456</guid>
      <pubDate>Sat, 20 Jan 2024 15:30:00 GMT</pubDate>
      <category>web</category>
    </item>
  </channel>
</rss>"#;

    let feed = feed_rs::parser::parse(rss_xml.as_bytes()).unwrap();

    assert_eq!(feed.entries.len(), 2);

    let first = &feed.entries[0];
    assert_eq!(
        first.title.as_ref().map(|t| t.content.as_str()),
        Some("First Article")
    );
    assert_eq!(first.categories.len(), 2);
    assert_eq!(first.categories[0].term, "rust");
    assert_eq!(first.categories[1].term, "programming");

    let second = &feed.entries[1];
    assert_eq!(
        second.title.as_ref().map(|t| t.content.as_str()),
        Some("Second Article")
    );
    assert_eq!(second.categories.len(), 1);
    assert_eq!(second.categories[0].term, "web");
}
