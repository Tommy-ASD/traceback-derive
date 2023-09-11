//! # `traceback-derive`
//!
//! `traceback-derive` is a procedural macro crate designed to enhance the functionality of the
//! `traceback-error` crate by providing custom macros for streamlined error handling and tracebacks in Rust.
//!
//! ## Usage
//!
//! To use `traceback-derive` in your Rust project, follow these steps:
//!
//! 1. Add `traceback-derive` and `traceback-error` as dependencies in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! traceback-derive = "0.1.1"
//! traceback-error = "0.1.5"
//! ```
//!
//! The `#[traceback]` attribute enhances the function with traceback capabilities, making it easier to handle errors
//! and capture detailed trace information.
//!
//! 2. Apply the `traceback` macro to your function to create and handle errors with tracebacks:
//!
//! ```rust
//! #[traceback_derive::traceback]
//! fn my_function() -> Result<(), traceback_error::TracebackError> {
//!     // Your code here
//!     risky_function()?;
//!     // ...
//! }
//! ```
//!
//! The `traceback!` macro simplifies error creation and captures relevant context information.
//!
//! ## Examples
//!
//! Here's an example of how `traceback-derive` simplifies error handling compared to using `traceback-error` directly:
//!
//! **Without `traceback-derive` (using `traceback-error` directly):**
//!
//! ```rust
//! use traceback_error::{traceback, TracebackError};
//!
//! fn main() {
//!     match caller_of_tasks() {
//!         Ok(_) => {}
//!         Err(e) => {
//!             traceback!(e, "One of the tasks failed");
//!         }
//!     }
//! }
//!
//! fn task_that_may_fail() -> Result<(), TracebackError> {
//!     return Err(traceback!("task_that_may_fail failed"));
//! }
//!
//! fn other_task_that_may_fail() -> Result<(), TracebackError> {
//!     return Err(traceback!("other_task_that_may_fail failed"));
//! }
//!
//! fn caller_of_tasks() -> Result<(), TracebackError> {
//!     match task_that_may_fail() {
//!         Ok(_) => {}
//!         Err(e) => {
//!             return Err(traceback!(err e));
//!         }
//!     };
//!     match other_task_that_may_fail() {
//!         Ok(_) => {}
//!         Err(e) => {
//!             return Err(traceback!(err e));
//!         }
//!     };
//!     Ok(())
//! }
//! ```
//!
//! **With `traceback-derive`:**
//!
//! ```rust
//! use traceback_error::{traceback, TracebackError};
//!
//! fn main() {
//!     match caller_of_tasks() {
//!         Ok(_) => {}
//!         Err(e) => {
//!             traceback!(e, "One of the tasks failed");
//!         }
//!     }
//! }
//!
//! fn task_that_may_fail() -> Result<(), TracebackError> {
//!     return Err(traceback!("task_that_may_fail failed"));
//! }
//!
//! fn other_task_that_may_fail() -> Result<(), TracebackError> {
//!     return Err(traceback!("other_task_that_may_fail failed"));
//! }
//!
//! #[traceback_derive::traceback]
//! fn caller_of_tasks() -> Result<(), TracebackError> {
//!     task_that_may_fail()?;
//!     other_task_that_may_fail()?;
//!     Ok(())
//! }
//! ```
//!
//! The two code snippets are equivalent when expanded, but `traceback-derive` simplifies error handling and capture.
//!
//! ## Contribution
//!
//! Contributions are welcome! Feel free to open issues or pull requests on the GitHub repository.
//! This project is still in very early development, and proper contribution guidelines have not yet been established.
//!
//! ## License
//!
//! This crate is dual-licensed under the [MIT License](LICENSE-MIT) and the [Apache License, Version 2.0](LICENSE-APACHE-2.0).
//! You may choose either of these licenses when using this crate.
//! See the [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE-2.0](LICENSE-APACHE-2.0) files for the full text of the licenses.
//!
//! ## GitHub Repository
//!
//! For more information and to contribute to the development of `traceback-derive`, visit the
//! [GitHub repository](https://github.com/Tommy-ASD/traceback-derive).
//!

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::ExprIndex;
use syn::{parse_macro_input, visit_mut::VisitMut, Expr};

#[proc_macro_attribute]
pub fn traceback(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(input as syn::ItemFn);

    let mut visitor = TracingVisitor;
    visitor.visit_item_fn_mut(&mut function);

    TokenStream::from(quote! { #function })
}

struct TracingVisitor;

impl VisitMut for TracingVisitor {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Try(expr_try) => {
                let span = expr_try.question_token.span();
                let inner_expr = &expr_try.expr;
                let new_expr = syn::parse2(quote_spanned! { span=>{
                    match #inner_expr {
                        Ok(val) => Ok(val),
                        Err(e) => Err(traceback!(err e))
                    }
                }?
                })
                .expect("Failed to create traceback match expression");

                *expr = new_expr;
            }
            Expr::Index(index) => {
                // Extract the parts of the index expression
                let ExprIndex {
                    attrs: _,
                    expr: inner_expr,
                    bracket_token: _,
                    index,
                } = index.clone();

                // Create a new expression for safe indexing
                let safe_indexing_expr = quote_spanned!(expr.span() =>
                    match #inner_expr.get(#index) {
                        Some(value) => value,
                        None => {
                            return Err(traceback!(format!("Error while indexing into {} in variable {:?}", #index, #inner_expr)));
                        },
                    }
                );

                // Replace the current expression with the safe indexing expression
                *expr = syn::parse2(safe_indexing_expr).unwrap();
            }
            _ => {
                syn::visit_mut::visit_expr_mut(self, expr);
            }
        }
    }
}
