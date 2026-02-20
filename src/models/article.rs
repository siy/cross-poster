use serde::{Deserialize, Serialize};

/// Lightweight article summary for list output
#[derive(Debug, Clone)]
pub struct ArticleSummary {
    pub id: String,
    pub title: String,
    pub url: String,
    pub published_at: String,
    pub tags: Vec<String>,
}

/// Internal representation of an article
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    /// Article title
    pub title: String,

    /// Article content in markdown format
    pub content: String,

    /// Optional tags/keywords for the article
    pub tags: Vec<String>,

    /// Optional canonical URL (original publication location)
    pub canonical_url: Option<String>,

    /// Optional publication status (published, draft, etc.)
    pub published: bool,

    /// Optional cover image URL
    pub cover_image: Option<String>,

    /// Optional article description/summary
    pub description: Option<String>,
}

impl Article {
    /// Create a new article with required fields
    pub fn new(title: String, content: String) -> Self {
        Self {
            title,
            content,
            tags: Vec::new(),
            canonical_url: None,
            published: true,
            cover_image: None,
            description: None,
        }
    }

    /// Builder pattern: add tags
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Builder pattern: set canonical URL
    pub fn with_canonical_url(mut self, url: String) -> Self {
        self.canonical_url = Some(url);
        self
    }

    /// Builder pattern: set publication status
    pub fn with_published(mut self, published: bool) -> Self {
        self.published = published;
        self
    }

    /// Builder pattern: set cover image
    pub fn with_cover_image(mut self, url: String) -> Self {
        self.cover_image = Some(url);
        self
    }

    /// Builder pattern: set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}
