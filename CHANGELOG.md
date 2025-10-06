# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- GitHub Actions CI pipeline for automated testing, linting, and formatting
- GitHub Actions release workflow for multi-platform binary builds
- Automated install script (`install.sh`) for Linux and macOS
- Pre-built binaries for Linux (x86_64), macOS (Intel + ARM), and Windows (x86_64)
- SHA256 checksum generation and verification for releases
- Quick install method via curl
- CLAUDE.md documentation for repository context
- `--format` flag to support HTML content format for Medium posts (markdown/html)
- Automatic title prepending to Medium content (Medium API requirement)
- Markdown-to-HTML conversion for better code block rendering on Medium
- Content size validation (1MB limit) for Medium posts
- Warnings when tags are truncated (dev.to: 4 max, Medium: 5 max)
- Detailed error context when API submissions fail (includes title, tags, content length, format)
- API key validation to prevent use of placeholder values
- Smart title extraction: automatically extract title from first H1 heading if not in frontmatter
- Title consistency validation: detect and prevent mismatches between frontmatter title and H1 heading

### Changed
- Enhanced README.md with comprehensive installation options
- Added release process documentation for maintainers
- Improved title prepending logic to check for any H1 heading (not exact match)
- Removed unused `user_id` field from Medium configuration

### Fixed
- Medium posts now include title in article content (previously only in metadata)
- Improved code block handling for Medium by supporting HTML format option
- YAML frontmatter now properly handles titles and descriptions with colons (requires quotes)
- Race condition in config file initialization using atomic file creation
- XSS vulnerability by disabling raw HTML parsing in markdown-to-HTML conversion
- dev.to API v1 authentication by adding required `Accept: application/vnd.forem.api-v1+json` header
- dev.to API 403 errors by adding required `User-Agent` header to all requests
- dev.to tag validation errors by automatically sanitizing tags (removes hyphens and special characters)

### Security
- HTML conversion explicitly disables raw HTML passthrough to prevent XSS attacks
- API key validation prevents accidental use of placeholder credentials

## [0.1.0] - 2025-10-04

### Added
- Initial release of article-cross-poster CLI tool
- Support for posting articles to dev.to platform
- Support for posting articles to Medium platform
- Markdown parsing with YAML frontmatter support
- AI artifact cleaning (emojis, smart quotes, dashes, etc.)
- Configuration management with secure local storage
- Import articles from dev.to URLs
- Preview command to see processed content before posting
- Dry run mode for testing without actual posting
- Command-line options for overriding tags and canonical URLs
- Comprehensive integration tests
- Example markdown files demonstrating all features
- Full documentation in README.md
- Detailed configuration guide in config.example.toml

### Security
- Config file permissions set to 0600 on Unix systems
- Path traversal prevention for file operations
- Secure credential storage in local config file
