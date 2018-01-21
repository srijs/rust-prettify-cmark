//! Pretty-printing for [CommonMark](http://commonmark.org/) documents.
//!
//! ## Simple API
//!
//! For simple use-cases, the [`prettify`](./fn.prettify.html) function
//! can be used to parse and pretty-print a CommonMark document.
//!
//! ```rust
//! use prettify_cmark::prettify;
//!
//! let output = prettify("Lorem __ipsum__ dolor `sit` amet!");
//!
//! assert_eq!(output,  "Lorem **ipsum** dolor `sit` amet!")
//! ```
//!
//! ## Advanced API
//!
//! For more advanced use-cases, this crate is designed to work well together
//! with the [`pulldown-cmark`](https://crates.io/crates/pulldown-cmark) crate.
//!
//! It provides a [`PrettyPrinter`](./struct.PrettyPrinter.html) which wraps
//! around an output type (such as `String`), and into which events can be
//! pushed that have been obtained from `pulldown-cmark`.
//!
//! ```rust
//! # extern crate pulldown_cmark;
//! # extern crate prettify_cmark;
//! use pulldown_cmark::Parser;
//! use prettify_cmark::PrettyPrinter;
//!
//! # fn main() {
//! let events = Parser::new("Lorem _ipsum_ dolor `sit`.");
//! let mut printer = PrettyPrinter::default();
//! printer.push_events(events).unwrap();
//!
//! assert_eq!(printer.into_inner(), "Lorem *ipsum* dolor `sit`.")
//! # }
//! ```

pub extern crate pulldown_cmark;

use std::fmt::{Display, Formatter, Result};

use pulldown_cmark::Parser;

mod writer;
mod printer;

#[cfg(test)]
mod tests;

pub use printer::PrettyPrinter;

/// Parses a CommonMark document and returns it as a pretty printed string.
///
/// # Examples
///
/// ```rust
/// # use prettify_cmark::prettify;
/// let output = prettify("Lorem __ipsum__ dolor `sit` amet!");
/// assert_eq!(output,  "Lorem **ipsum** dolor `sit` amet!");
/// ```
pub fn prettify(source: &str) -> String {
    PrettyDisplay(source).to_string()
}

/// Wrapper that will pretty print the wrapped document when formatted
/// via `Display`.
///
/// # Examples
///
/// Via `to_string`:
///
/// ```rust
/// # use prettify_cmark::PrettyDisplay;
/// let output = PrettyDisplay("Lorem __ipsum__ dolor `sit` amet!").to_string();
/// assert_eq!(output,  "Lorem **ipsum** dolor `sit` amet!");
/// ```
///
/// Via `format!`:
///
/// ```rust
/// # use prettify_cmark::PrettyDisplay;
/// let output = format!("My document: {}", PrettyDisplay("Lorem __ipsum__ dolor `sit` amet!"));
/// assert_eq!(output,  "My document: Lorem **ipsum** dolor `sit` amet!");
/// ```
pub struct PrettyDisplay<T>(pub T);

impl<T: AsRef<str>> Display for PrettyDisplay<T> {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let events = Parser::new(self.0.as_ref());
        let mut pretty_printer = PrettyPrinter::new(fmt);
        pretty_printer.push_events(events)
    }
}
