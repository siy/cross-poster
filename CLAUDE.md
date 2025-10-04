# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust CLI tool for cross-posting markdown articles to dev.to and Medium with AI artifact cleanup. Pure CLI, stateless operation with secure local config.

## Build, Test, and Development Commands

```bash
# Build
cargo build --release

# Run tests
cargo test

# Run integration tests
cargo test --test integration_tests

# Run a single test
cargo test test_name

# Lint
cargo clippy

# Format
cargo fmt

# Run the CLI
cargo run -- <subcommand>
```

## Code Architecture

### Module Structure

The codebase follows a clean separation of concerns:

- **`cli/`**: Command-line interface layer
  - `args.rs`: Clap-based argument parsing, defines `Commands`, `Platform` enum, and `ConfigAction`
  - `config.rs`: Configuration management - loads/saves TOML config to `~/.config/article-cross-poster/config.toml`, sets file permissions to 0600 on Unix

- **`models/`**: Core data structures
  - `article.rs`: Internal `Article` representation with builder pattern for optional fields (tags, canonical_url, cover_image, description, published status)

- **`parsers/`**: Content processing
  - `markdown.rs`: YAML frontmatter parsing with `gray_matter`, requires `title` field
  - `cleaner.rs`: AI artifact removal (emojis, smart quotes, em/en dashes, zero-width characters)
  - `devto.rs`: Parse dev.to URLs and extract article IDs
  - `sanitizer.rs`: Input validation and security

- **`platforms/`**: Publishing clients
  - `devto.rs`: dev.to API client (max 4 tags, uses `api-key` header)
  - `medium.rs`: Medium API client (max 5 tags, uses Bearer token auth, requires user ID from `/v1/me`)

### Key Architectural Patterns

1. **Platform Abstraction**: Both `DevToClient` and `MediumClient` implement async `publish_article()` taking `&Article`, returning URL
2. **Builder Pattern**: `Article::new()` followed by chained `.with_*()` methods
3. **Error Context**: Heavy use of `anyhow::Context` for error wrapping throughout
4. **Security**: Path canonicalization in `load_article()` prevents path traversal attacks
5. **Credential Flow**: Config loaded from TOML → clients constructed with credentials → publish methods called

## Platform-Specific Details

### dev.to API
- Endpoint: `https://dev.to/api/articles`
- Auth: `api-key` header
- Max tags: 4
- Field mapping: `main_image` for cover, `body_markdown` for content

### Medium API
- Endpoint: `https://api.medium.com/v1`
- Auth: `Bearer {token}`
- Requires fetching user ID from `/v1/me` before publishing
- Max tags: 5
- Field mapping: `contentFormat: markdown`, `publishStatus: public|draft`

## Article Format

Markdown with required YAML frontmatter:
```yaml
---
title: Required Title
tags: [optional, array]
canonical_url: optional_string
published: bool (default: true)
cover_image: optional_url
description: optional_string
---
```

## Testing Strategy

Integration tests in `tests/integration_tests.rs` cover:
- Markdown parsing (valid/invalid frontmatter)
- AI artifact cleaning transformations
- Config file creation and permissions
- URL parsing for dev.to imports

Unit tests embedded in source files test individual functions.

## Security Considerations

- API credentials stored in **plain text** at `~/.config/article-cross-poster/config.toml`
- File permissions automatically set to 0600 (user read/write only) on Unix
- Path traversal prevention via `canonicalize()` in file loading (src/main.rs:189)
- Input sanitization for dev.to URL parsing

## CI/CD Pipeline

### GitHub Actions Workflows

**CI Workflow** (`.github/workflows/ci.yml`):
- Triggers on push to main/master and pull requests
- Jobs: test, lint (clippy), format check
- Uses cargo caching for faster builds
- Runs both unit tests and integration tests

**Release Workflow** (`.github/workflows/release.yml`):
- Triggers on git tags matching `v*` pattern (e.g., `v0.1.0`)
- Builds cross-platform binaries using matrix strategy:
  - Linux: x86_64-unknown-linux-gnu (ubuntu-latest)
  - macOS Intel: x86_64-apple-darwin (macos-13)
  - macOS ARM: aarch64-apple-darwin (macos-14)
  - Windows: x86_64-pc-windows-msvc (windows-latest)
- Creates GitHub release with all platform archives
- Generates SHA256 checksums for each binary
- Archives include: binary, README.md, LICENSE (if present)

### Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with new version and changes
3. Commit changes: `git commit -m "chore: release v0.2.0"`
4. Create and push tag: `git tag v0.2.0 && git push origin v0.2.0`
5. GitHub Actions automatically builds and publishes release

### Installation Script

`install.sh` provides automated installation:
- Detects OS (Linux/macOS) and architecture (x86_64/aarch64)
- Downloads appropriate binary from GitHub releases
- Verifies SHA256 checksum if available
- Installs to `/usr/local/bin` or `~/.local/bin` (fallback)
- Configurable via environment variables: `INSTALL_DIR`, `VERSION`, `GITHUB_USER`

## Changelog Policy

Before committing significant changes, update CHANGELOG.md following [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format.
