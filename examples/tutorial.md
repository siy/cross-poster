---
title: Advanced Rust Patterns for Error Handling
tags: [rust, error-handling, patterns, best-practices]
published: true
description: Explore advanced error handling patterns in Rust using Result, anyhow, and thiserror
---

# Advanced Rust Patterns for Error Handling

Error handling is a critical aspect of robust software development. Rust provides powerful tools for managing errors in a type-safe way.

## The Result Type

Rust's `Result<T, E>` enum is the foundation of error handling:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## Basic Error Handling

Simple error propagation with the `?` operator:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

## Using anyhow for Applications

For applications, `anyhow` provides convenient error handling:

```rust
use anyhow::{Context, Result};

fn process_data(input: &str) -> Result<()> {
    let data = parse_input(input)
        .context("Failed to parse input data")?;
    
    let result = transform_data(data)
        .context("Failed to transform data")?;
    
    save_result(result)
        .context("Failed to save result")?;
    
    Ok(())
}
```

## Custom Errors with thiserror

For libraries, define custom error types with `thiserror`:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("Invalid data format: {0}")]
    InvalidFormat(String),
    
    #[error("Data not found")]
    NotFound,
    
    #[error("IO error")]
    Io(#[from] std::io::Error),
}
```

## Error Recovery Patterns

Handle errors with fallback values:

```rust
fn get_config_or_default() -> Config {
    load_config()
        .unwrap_or_else(|e| {
            eprintln!("Warning: {}, using defaults", e);
            Config::default()
        })
}
```

## Combining Error Types

Use trait objects for multiple error types:

```rust
use std::error::Error;

fn complex_operation() -> Result<(), Box<dyn Error>> {
    // Can return different error types
    let file = File::open("data.txt")?;
    let parsed: Data = serde_json::from_reader(file)?;
    Ok(())
}
```

## Testing Error Cases

Write tests for error conditions:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_input() {
        let result = parse_input("invalid");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_error_message() {
        let err = parse_input("bad").unwrap_err();
        assert!(err.to_string().contains("Invalid"));
    }
}
```

## Best Practices

1. **Use Result for recoverable errors**: Don't panic for expected failures
2. **Provide context**: Use `.context()` to add helpful error messages
3. **Custom errors for libraries**: Define specific error types
4. **anyhow for applications**: Simplifies error handling in binaries
5. **Document errors**: Specify which errors functions can return

## Common Patterns

### Early Return Pattern

```rust
fn validate_and_process(input: &str) -> Result<Output> {
    if input.is_empty() {
        return Err(anyhow!("Input cannot be empty"));
    }
    
    let validated = validate(input)?;
    let processed = process(validated)?;
    Ok(processed)
}
```

### Match for Fine-Grained Control

```rust
match risky_operation() {
    Ok(value) => println!("Success: {}", value),
    Err(e) if e.is_retriable() => retry_operation(),
    Err(e) => eprintln!("Fatal error: {}", e),
}
```

### Logging Errors

```rust
fn process_with_logging(item: Item) -> Result<()> {
    transform_item(item).map_err(|e| {
        error!("Failed to transform item: {}", e);
        e
    })?;
    Ok(())
}
```

## Conclusion

Effective error handling makes your Rust programs more robust and maintainable. Choose the right tools for your use case: `Result` and `Option` for basics, `anyhow` for applications, and `thiserror` for libraries.

## Further Reading

- [The Rust Programming Language - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [anyhow documentation](https://docs.rs/anyhow/)
- [thiserror documentation](https://docs.rs/thiserror/)
