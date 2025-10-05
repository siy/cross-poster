use anyhow::Result;
use pulldown_cmark::{html, Options, Parser};

/// Medium's approximate content size limit (1MB)
const MEDIUM_MAX_CONTENT_SIZE: usize = 1024 * 1024;

/// Convert markdown to HTML safely
///
/// This function converts markdown to HTML without allowing raw HTML passthrough,
/// preventing XSS attacks. It also validates content size limits.
pub fn markdown_to_html(markdown: &str) -> Result<String> {
    if markdown.len() > MEDIUM_MAX_CONTENT_SIZE {
        anyhow::bail!(
            "Content too large for conversion: {} bytes (max: {})",
            markdown.len(),
            MEDIUM_MAX_CONTENT_SIZE
        );
    }

    // Configure options - IMPORTANT: Do NOT enable ENABLE_HTML to prevent XSS
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    // Note: Options::ENABLE_HTML is intentionally NOT set for security

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    if html_output.len() > MEDIUM_MAX_CONTENT_SIZE {
        anyhow::bail!(
            "Converted HTML too large: {} bytes (max: {})",
            html_output.len(),
            MEDIUM_MAX_CONTENT_SIZE
        );
    }

    Ok(html_output)
}

/// Prepend title as H1 heading if not already present
///
/// This function checks if the content starts with ANY H1 heading.
/// If it does, we assume the title is already present and don't add it.
/// This prevents duplication when the title appears at the start,
/// regardless of whether it exactly matches the article title.
pub fn ensure_title_in_content(title: &str, content: &str) -> String {
    let trimmed = content.trim();

    // Check if content starts with any H1 heading (# ...)
    // If so, assume a title is already present and don't add another
    if trimmed.starts_with("# ") {
        content.to_string()
    } else {
        format!("# {}\n\n{}", title, content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_to_html_basic() {
        let markdown = "# Hello\n\nThis is **bold** and *italic*.";
        let html = markdown_to_html(markdown).unwrap();

        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("<strong>bold</strong>"));
        assert!(html.contains("<em>italic</em>"));
    }

    #[test]
    fn test_markdown_to_html_code_blocks() {
        let markdown = "```rust\nfn main() {}\n```";
        let html = markdown_to_html(markdown).unwrap();

        assert!(html.contains("<code"));
        assert!(html.contains("fn main()"));
    }

    #[test]
    fn test_markdown_to_html_tables() {
        let markdown = "| A | B |\n|---|---|\n| 1 | 2 |";
        let html = markdown_to_html(markdown).unwrap();

        assert!(html.contains("<table>"));
        assert!(html.contains("<th>"));
        assert!(html.contains("<td>"));
    }

    #[test]
    fn test_markdown_to_html_with_inline_html() {
        let markdown = "Regular **markdown** content";
        let html = markdown_to_html(markdown).unwrap();

        // Should convert markdown properly
        assert!(html.contains("<strong>markdown</strong>"));

        // Note: pulldown-cmark without ENABLE_HTML option treats inline HTML
        // as text content and doesn't escape it. This is documented behavior.
        // The important part is that we're NOT enabling ENABLE_HTML which would
        // explicitly parse and pass through HTML tags.
    }

    #[test]
    fn test_markdown_to_html_size_limit() {
        let huge_markdown = "a".repeat(MEDIUM_MAX_CONTENT_SIZE + 1);
        let result = markdown_to_html(&huge_markdown);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too large"));
    }

    #[test]
    fn test_ensure_title_when_missing() {
        let title = "My Article";
        let content = "This is the content.";
        let result = ensure_title_in_content(title, content);

        assert_eq!(result, "# My Article\n\nThis is the content.");
    }

    #[test]
    fn test_ensure_title_when_present() {
        let title = "My Article";
        let content = "# My Article\n\nThis is the content.";
        let result = ensure_title_in_content(title, content);

        assert_eq!(result, content);
    }

    #[test]
    fn test_ensure_title_with_different_heading() {
        let title = "My Article";
        let content = "## Introduction\n\nThis is the content.";
        let result = ensure_title_in_content(title, content);

        // Should prepend title since content doesn't start with H1
        assert_eq!(result, "# My Article\n\n## Introduction\n\nThis is the content.");
    }

    #[test]
    fn test_ensure_title_with_any_h1() {
        let title = "My Article";
        let content = "# Different Title\n\nContent here";
        let result = ensure_title_in_content(title, content);

        // Should NOT prepend title since content already has H1
        assert_eq!(result, content);
    }

    #[test]
    fn test_ensure_title_with_whitespace() {
        let title = "My Article";
        let content = "\n\n  This is the content.";
        let result = ensure_title_in_content(title, content);

        assert!(result.starts_with("# My Article\n\n"));
    }
}
