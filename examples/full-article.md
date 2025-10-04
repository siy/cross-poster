---
title: "Building a CLI Tool in Rust: A Complete Guide"
tags: [rust, cli, tutorial, development]
canonical_url: https://yourblog.com/building-cli-tool-rust
published: true
cover_image: https://images.unsplash.com/photo-1629654297299-c8506221ca97
description: Learn how to build a production-ready command-line interface tool using Rust, from setup to distribution.
---

# Building a CLI Tool in Rust: A Complete Guide

In this comprehensive tutorial, we'll build a production-ready command-line interface (CLI) tool using Rust. We'll cover everything from project setup to distribution.

## Prerequisites

Before starting, ensure you have:

- Rust installed (via rustup)
- Basic knowledge of Rust syntax
- Familiarity with terminal/command line

## Project Setup

Create a new Rust project:

```bash
cargo new my-cli-tool
cd my-cli-tool
```

## Adding Dependencies

We'll use `clap` for argument parsing. Add to `Cargo.toml`:

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
anyhow = "1.0"
```

## Building the CLI

Define your command-line interface:

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "my-cli")]
#[command(about = "A sample CLI tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Process a file
    Process {
        /// Input file path
        file: String,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Process { file } => {
            println!("Processing file: {}", file);
        }
    }
}
```

## Error Handling

Use `anyhow` for convenient error handling:

```rust
use anyhow::{Context, Result};

fn process_file(path: &str) -> Result<()> {
    let content = std::fs::read_to_string(path)
        .context("Failed to read file")?;
    
    // Process content here
    
    Ok(())
}
```

## Testing

Add unit tests for your CLI logic:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processing() {
        // Your tests here
    }
}
```

## Building for Release

Build an optimized binary:

```bash
cargo build --release
```

The binary will be in `target/release/`.

## Distribution

Consider these distribution methods:

1. **Direct binary**: Share the compiled executable
2. **Cargo install**: Publish to crates.io
3. **Package managers**: Create packages for apt, brew, etc.

## Best Practices

1. **Use structured logging**: Consider `env_logger` or `tracing`
2. **Handle signals**: Gracefully handle Ctrl+C
3. **Provide help text**: Use clap's built-in help generation
4. **Version your tool**: Keep track of versions with semantic versioning
5. **Test on multiple platforms**: Linux, macOS, Windows

## Conclusion

You now have a solid foundation for building CLI tools in Rust. The combination of Rust's performance, safety, and ecosystem makes it an excellent choice for command-line applications.

## Resources

- [Rust CLI Book](https://rust-cli.github.io/book/)
- [clap Documentation](https://docs.rs/clap/)
- [The Rust Book](https://doc.rust-lang.org/book/)

Happy coding!
