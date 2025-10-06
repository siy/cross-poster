use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::cli::ContentFormat;
use crate::models::Article;
use crate::parsers::{ensure_title_in_content, markdown_to_html};

/// Maximum number of tags allowed by Medium
const MEDIUM_MAX_TAGS: usize = 5;

/// Medium API client
pub struct MediumClient {
    client: Client,
    access_token: String,
    base_url: String,
}

/// Response from Medium GET /v1/me
#[derive(Debug, Deserialize)]
struct MediumUserResponse {
    data: MediumUser,
}

/// Medium user data
#[derive(Debug, Deserialize)]
struct MediumUser {
    id: String,
}

/// Request body for Medium POST /v1/users/{userId}/posts
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MediumPublishRequest {
    title: String,
    content_format: MediumContentFormat,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    canonical_url: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
    publish_status: PublishStatus,
}

/// Content format for Medium API
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
#[allow(dead_code)]
enum MediumContentFormat {
    Markdown,
    Html,
}

/// Publication status for Medium articles
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
#[allow(dead_code)]
enum PublishStatus {
    Public,
    Draft,
    Unlisted,
}

/// Response from Medium POST /v1/users/{userId}/posts
#[derive(Debug, Deserialize)]
struct MediumPublishResponse {
    data: MediumPost,
}

/// Medium post data
#[derive(Debug, Deserialize)]
struct MediumPost {
    url: String,
}

impl MediumClient {
    /// Create a new Medium client
    pub fn new(access_token: String) -> Self {
        Self {
            client: Client::new(),
            access_token,
            base_url: "https://api.medium.com/v1".to_string(),
        }
    }

    /// Get the authenticated user's ID
    async fn get_user_id(&self) -> Result<String> {
        let url = format!("{}/me", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await
            .context("Failed to send request to Medium API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();

            let error_msg = if status == 401 {
                "Invalid access token - check your Medium credentials"
            } else {
                "Failed to authenticate with Medium API"
            };

            anyhow::bail!("{} (status {}): {}", error_msg, status, error_text);
        }

        let user_response: MediumUserResponse = response
            .json()
            .await
            .context("Failed to parse Medium user response")?;

        Ok(user_response.data.id)
    }

    /// Publish an article to Medium with specified format
    pub async fn publish_article(
        &self,
        article: &Article,
        format: &ContentFormat,
    ) -> Result<String> {
        // First, get the user ID
        let user_id = self.get_user_id().await?;

        let url = format!("{}/users/{}/posts", self.base_url, user_id);

        // Medium has a max of 5 tags - warn if truncating
        let tags: Vec<String> = article.tags.iter().take(MEDIUM_MAX_TAGS).cloned().collect();
        let tags_str = tags.join(", "); // Save before moving
        let tags_len = tags.len();

        if article.tags.len() > MEDIUM_MAX_TAGS {
            eprintln!(
                "⚠️  Warning: Medium only supports {} tags. Truncating from {} to {} tags.",
                MEDIUM_MAX_TAGS,
                article.tags.len(),
                MEDIUM_MAX_TAGS
            );
            eprintln!("   Included: {}", tags_str);
            eprintln!(
                "   Excluded: {}",
                article.tags[MEDIUM_MAX_TAGS..].join(", ")
            );
        }

        let publish_status = if article.published {
            PublishStatus::Public
        } else {
            PublishStatus::Draft
        };

        // Ensure title is in content (Medium API requires this)
        let content_with_title = ensure_title_in_content(&article.title, &article.content);

        // Convert format based on user preference
        let (content_format, content) = match format {
            ContentFormat::Markdown => (MediumContentFormat::Markdown, content_with_title),
            ContentFormat::Html => {
                let html = markdown_to_html(&content_with_title)
                    .context("Failed to convert markdown to HTML")?;
                (MediumContentFormat::Html, html)
            }
        };

        // Save content length for error reporting before moving content
        let content_len = content.len();

        let request_body = MediumPublishRequest {
            title: article.title.clone(),
            content_format,
            content,
            canonical_url: article.canonical_url.clone(),
            tags,
            publish_status,
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .context("Failed to send publish request to Medium API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();

            let error_msg = if status == 401 {
                "Invalid access token - check your Medium credentials"
            } else if status == 429 {
                "Rate limit exceeded - please try again later"
            } else if status == 400 {
                "Article validation failed - check title and content"
            } else {
                "API request failed"
            };

            anyhow::bail!(
                "{} (status {})\n\
                \n\
                Server Response:\n\
                {}\n\
                \n\
                Article Details:\n\
                  Title: '{}'\n\
                  Format: {}\n\
                  Tags: {} ({})\n\
                  Content length: {} chars",
                error_msg,
                status,
                if error_text.is_empty() {
                    "(no response body)"
                } else {
                    &error_text
                },
                article.title,
                format,
                tags_len,
                tags_str,
                content_len
            );
        }

        let publish_response: MediumPublishResponse = response
            .json()
            .await
            .context("Failed to parse Medium publish response")?;

        Ok(publish_response.data.url)
    }
}
