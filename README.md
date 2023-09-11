# `traceback-derive`

`traceback-derive` is a procedural macro crate designed to enhance the functionality of the
`traceback-error` crate by providing custom macros for streamlined error handling and tracebacks in Rust.

## Usage

To use `traceback-derive` in your Rust project, follow these steps:

1. Add `traceback-derive` and `traceback-error` as dependencies in your `Cargo.toml`:

```toml
[dependencies]
traceback-derive = "0.1.1"
traceback-error = "0.1.5"
```

The `#[traceback]` attribute enhances the function with traceback capabilities, making it easier to handle errors
and capture detailed trace information.

2. Apply the `traceback` macro to your function to create and handle errors with tracebacks:

```rust
#[traceback_derive::traceback]
fn my_function() -> Result<(), traceback_error::TracebackError> {
    // Your code here
    risky_function()?;
    // ...
}
```

The `traceback!` macro simplifies error creation and captures relevant context information.

## Examples

Here's an example of how `traceback-derive` simplifies error handling compared to using `traceback-error` directly:

**Without `traceback-derive` (using `traceback-error` directly):**

```rust
use traceback_error::{traceback, TracebackError};

fn main() {
    match caller_of_tasks() {
        Ok(_) => {}
        Err(e) => {
            traceback!(e, "One of the tasks failed");
        }
    }
}

fn task_that_may_fail() -> Result<(), TracebackError> {
    return Err(traceback!("task_that_may_fail failed"));
}

fn other_task_that_may_fail() -> Result<(), TracebackError> {
    return Err(traceback!("other_task_that_may_fail failed"));
}

fn caller_of_tasks() -> Result<(), TracebackError> {
    match task_that_may_fail() {
        Ok(_) => {}
        Err(e) => {
            return Err(traceback!(err e));
        }
    };
    match other_task_that_may_fail() {
        Ok(_) => {}
        Err(e) => {
            return Err(traceback!(err e));
        }
    };
    Ok(())
}
```

**With `traceback-derive`:**

```rust
use traceback_error::{traceback, TracebackError};

fn main() {
    match caller_of_tasks() {
        Ok(_) => {}
        Err(e) => {
            traceback!(e, "One of the tasks failed");
        }
    }
}

fn task_that_may_fail() -> Result<(), TracebackError> {
    return Err(traceback!("task_that_may_fail failed"));
}

fn other_task_that_may_fail() -> Result<(), TracebackError> {
    return Err(traceback!("other_task_that_may_fail failed"));
}

#[traceback_derive::traceback]
fn caller_of_tasks() -> Result<(), TracebackError> {
    task_that_may_fail()?;
    other_task_that_may_fail()?;
    Ok(())
}
```

The two code snippets are equivalent when expanded, but `traceback-derive` simplifies error handling and capture.

## Contribution

Contributions are welcome! Feel free to open issues or pull requests on the GitHub repository.
This project is still in very early development, and proper contribution guidelines have not yet been established.

## License

This crate is dual-licensed under the [MIT License](LICENSE-MIT) and the [Apache License, Version 2.0](LICENSE-APACHE-2.0).
You may choose either of these licenses when using this crate.
See the [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE-2.0](LICENSE-APACHE-2.0) files for the full text of the licenses.

## GitHub Repository

For more information and to contribute to the development of `traceback-derive`, visit the
[GitHub repository](https://github.com/Tommy-ASD/traceback-derive).
