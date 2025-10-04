use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::models::Article;

/// dev.to API client
pub struct DevToClient {
    client: Client,
    api_key: String,
    base_url: String,
}

/// Response from dev.to GET /api/articles/{id}
#[derive(Debug, Deserialize)]
struct DevToArticleResponse {
    title: String,
    body_markdown: String,
    #[serde(default)]
    tags: Vec<String>,
    canonical_url: Option<String>,
    cover_image: Option<String>,
    description: Option<String>,
    published: bool,
}

/// Request body for dev.to POST /api/articles
#[derive(Debug, Serialize)]
struct DevToPublishRequest {
    article: DevToArticleData,
}

/// Article data for dev.to publishing
#[derive(Debug, Serialize)]
struct DevToArticleData {
    title: String,
    body_markdown: String,
    published: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    canonical_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    main_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    series: Option<String>,
}

impl DevToClient {
    /// Create a new dev.to client
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://dev.to/api".to_string(),
        }
    }

    /// Fetch an article from dev.to by ID
    pub async fn fetch_article(&self, article_id: &str) -> Result<Article> {
        let url = format!("{}/articles/{}", self.base_url, article_id);

        let response = self
            .client
            .get(&url)
            .header("api-key", &self.api_key)
            .send()
            .await
            .context("Failed to send request to dev.to API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("dev.to API error (status {}): {}", status, error_text);
        }

        let devto_article: DevToArticleResponse = response
            .json()
            .await
            .context("Failed to parse dev.to article response")?;

        Ok(Article {
            title: devto_article.title,
            content: devto_article.body_markdown,
            tags: devto_article.tags,
            canonical_url: devto_article.canonical_url,
            published: devto_article.published,
            cover_image: devto_article.cover_image,
            description: devto_article.description,
        })
    }

    /// Publish an article to dev.to
    pub async fn publish_article(&self, article: &Article) -> Result<String> {
        let url = format!("{}/articles", self.base_url);

        // dev.to has a max of 4 tags
        let tags: Vec<String> = article.tags.iter().take(4).cloned().collect();

        let request_body = DevToPublishRequest {
            article: DevToArticleData {
                title: article.title.clone(),
                body_markdown: article.content.clone(),
                published: article.published,
                tags,
                canonical_url: article.canonical_url.clone(),
                main_image: article.cover_image.clone(),
                description: article.description.clone(),
                series: None,
            },
        };

        let response = self
            .client
            .post(&url)
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .context("Failed to send publish request to dev.to API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();

            // Provide more specific error messages for common issues
            let error_msg = if status == 401 {
                "Invalid API key - check your dev.to credentials"
            } else if status == 429 {
                "Rate limit exceeded - please try again later"
            } else if status == 422 {
                "Article validation failed - check title, content, and tags"
            } else {
                "API request failed"
            };

            anyhow::bail!("{} (status {}): {}", error_msg, status, error_text);
        }

        #[derive(Deserialize)]
        struct PublishResponse {
            url: String,
        }

        let publish_response: PublishResponse = response
            .json()
            .await
            .context("Failed to parse dev.to publish response")?;

        Ok(publish_response.url)
    }
}
