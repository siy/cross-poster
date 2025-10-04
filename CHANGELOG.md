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

### Changed
- Enhanced README.md with comprehensive installation options
- Added release process documentation for maintainers

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
