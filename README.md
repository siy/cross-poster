# Article Cross-Poster

A pure CLI tool for cross-posting articles to dev.to and Medium with AI artifact cleanup.

## Features

- 📝 Post markdown articles to dev.to and Medium
- 🔗 Import articles directly from dev.to URLs
- 🧹 Clean AI-generated artifacts (emojis, smart quotes, etc.)
- 👀 Preview processed content before posting
- 🔒 Secure credential storage in local config file
- 🎯 Simple, lean, and stateless operation

## Installation

### Quick Install (Recommended)

**Linux and macOS:**

```bash
curl -fsSL https://raw.githubusercontent.com/siy/cross-poster/main/install.sh | sh
```

The installer will automatically detect your OS and architecture, download the appropriate binary, and install it to `/usr/local/bin` (or `~/.local/bin` if you don't have sudo access).

**Custom installation directory:**

```bash
curl -fsSL https://raw.githubusercontent.com/siy/cross-poster/main/install.sh | INSTALL_DIR=$HOME/bin sh
```

**Install specific version:**

```bash
curl -fsSL https://raw.githubusercontent.com/siy/cross-poster/main/install.sh | VERSION=0.1.0 sh
```

### Download Pre-built Binaries

Download the latest release for your platform from the [releases page](https://github.com/siy/cross-poster/releases/latest):

- **Linux (x86_64)**: `article-cross-poster-linux-x86_64.tar.gz`
- **macOS (Intel)**: `article-cross-poster-macos-x86_64.tar.gz`
- **macOS (Apple Silicon)**: `article-cross-poster-macos-aarch64.tar.gz`
- **Windows (x86_64)**: `article-cross-poster-windows-x86_64.zip`

Extract the archive and move the binary to a directory in your PATH:

```bash
# Example for Linux/macOS
tar -xzf article-cross-poster-*.tar.gz
sudo mv article-cross-poster /usr/local/bin/
```

### From Source

For developers or if you want to build from source:

```bash
git clone https://github.com/siy/cross-poster.git
cd cross-poster
cargo build --release
```

The binary will be available at `target/release/article-cross-poster`.

**Add to PATH:**

```bash
# Copy to a directory in your PATH
sudo cp target/release/article-cross-poster /usr/local/bin/
```

## Configuration

### Initialize Config

Create a configuration file with your API credentials:

```bash
article-cross-poster config init
```

This creates `~/.config/article-cross-poster/config.toml` with restrictive permissions (600 on Unix).

### Get API Credentials

#### dev.to API Key
1. Go to https://dev.to/settings/extensions
2. Generate an API key
3. Copy the key to your config file

#### Medium Access Token
1. Go to https://medium.com/me/settings/security
2. Generate an integration token
3. Get your user ID from your Medium profile URL (e.g., `@username` → find user ID)
4. Add both to your config file

### Edit Config

```bash
# Show config file location
article-cross-poster config path

# Edit the file manually
vim ~/.config/article-cross-poster/config.toml
```

Example config:

```toml
[dev_to]
api_key = "your_dev_to_api_key"

[medium]
access_token = "your_medium_access_token"
user_id = "your_medium_user_id"
```

### Verify Config

```bash
article-cross-poster config show
```

## Usage

### Post an Article

Post to a single platform:

```bash
article-cross-poster post -t devto article.md
```

Post to multiple platforms:

```bash
article-cross-poster post -t devto,medium article.md
```

### Clean AI Artifacts

Remove emojis, smart quotes, and other AI-generated formatting:

```bash
article-cross-poster post -t devto --clean-ai article.md
```

### Preview Before Posting

Preview how your article will look after processing:

```bash
article-cross-poster preview article.md
article-cross-poster preview --clean-ai article.md
```

### Import from dev.to

Fetch an article from dev.to and post it to Medium:

```bash
article-cross-poster post -t medium https://dev.to/username/article-slug
```

### Override Metadata

Override tags:

```bash
article-cross-poster post -t devto --tags rust,cli,tutorial article.md
```

Set canonical URL:

```bash
article-cross-poster post -t medium --canonical https://yourblog.com/article article.md
```

### Dry Run

Test without actually posting:

```bash
article-cross-poster post -t devto,medium --dry-run article.md
```

## Article Format

Articles must be in markdown format with YAML frontmatter:

```markdown
---
title: Your Article Title
tags: [rust, cli, tutorial]
canonical_url: https://yourblog.com/article
published: true
cover_image: https://example.com/cover.jpg
description: A brief description of your article
---

# Your Article Content

Write your article content here in markdown format.

## Code Examples

\```rust
fn main() {
    println!("Hello, world!");
}
\```
```

### Required Fields

- `title`: Article title (required)

### Optional Fields

- `tags`: Array of tags/keywords
- `canonical_url`: Original publication URL
- `published`: Publication status (default: true)
- `cover_image`: Cover image URL
- `description`: Article description/summary

## AI Artifact Cleaning

The `--clean-ai` flag removes common AI-generated formatting:

- **Emojis**: 👋 🌍 🚀 → (removed)
- **Smart quotes**: "text" → "text"
- **Smart apostrophes**: 'text' → 'text'
- **Em dashes**: — → --
- **En dashes**: – → -
- **Ellipsis**: … → ...
- **Zero-width characters**: (removed)

This is useful when content was generated or edited by AI tools and you want plain ASCII formatting.

## Examples

See the `examples/` directory for sample markdown files:

- `examples/basic-article.md` - Minimal article with required fields
- `examples/full-article.md` - Article with all optional fields
- `examples/tutorial.md` - Tutorial with code examples

## Troubleshooting

### Config file not found

```bash
article-cross-poster config init
```

### Invalid API credentials

Verify your config:

```bash
article-cross-poster config show
```

Check that you've copied the correct API keys from the platform settings.

### Permission denied on config file

On Unix systems, ensure proper permissions:

```bash
chmod 600 ~/.config/article-cross-poster/config.toml
```

### Failed to parse markdown

Ensure your markdown file has valid YAML frontmatter with a `title` field:

```yaml
---
title: Your Title Here
---
```

### Platform-specific errors

- **dev.to**: Verify your API key is active and has write permissions
- **Medium**: Ensure you're using an integration token (not OAuth) and correct user ID

## Security

⚠️ **WARNING**: API keys and tokens are stored in **PLAIN TEXT** in the config file.

- Config file permissions are set to 0600 (user read/write only) on Unix
- Never commit your config file to version control
- Keep your API keys private and rotate them regularly
- The tool is designed for local personal use only

## Development

### Run Tests

```bash
cargo test
```

### Run Linter

```bash
cargo clippy
```

### Format Code

```bash
cargo fmt
```

### Build for Release

```bash
cargo build --release
```

### Creating a Release

This project uses GitHub Actions for automated releases. To create a new release:

1. Update the version in `Cargo.toml`
2. Update `CHANGELOG.md` with the new version and changes
3. Commit the changes
4. Create and push a git tag:

```bash
git tag v0.2.0
git push origin v0.2.0
```

The release workflow will automatically:
- Build binaries for Linux, macOS (Intel + ARM), and Windows
- Create a GitHub release with all binaries
- Generate SHA256 checksums for verification

Users can then install the new version using the install script or download binaries directly from the releases page.

## License

MIT

## Contributing

Contributions welcome! Please open an issue or pull request.
