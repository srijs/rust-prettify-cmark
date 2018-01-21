# `prettify-cmark`

Pretty-printing for [CommonMark](http://commonmark.org/) documents.

[![Build Status](https://travis-ci.org/srijs/rust-prettify-cmark.svg?branch=master)](https://travis-ci.org/srijs/rust-prettify-cmark)
[![crates](http://meritbadge.herokuapp.com/prettify-cmark)](https://crates.io/crates/prettify-cmark)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
prettify-cmark = "0.1"
```

## Examples

```rust
extern crate pulldown_cmark;
extern crate prettify_cmark;

use pulldown_cmark::Parser;
use prettify_cmark::PrettyPrinter;

fn main() {
    let events = Parser::new("Lorem _ipsum_ dolor `sit`.");
    let mut printer = PrettyPrinter::default();
    printer.push_events(events).unwrap();

    assert_eq!(printer.into_inner(), "Lorem *ipsum* dolor `sit`.")
}
```
