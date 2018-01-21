use super::prettify;

#[test]
fn simple_paragraph() {
    let output = prettify("Lorem ipsum");
    assert_eq!(output, "Lorem ipsum");
}

#[test]
fn paragraph_with_softbreak() {
    let output = prettify("Lorem ipsum\ndolor sit");
    assert_eq!(output, "Lorem ipsum dolor sit");
}

#[test]
fn paragraph_with_hardbreak() {
    let output = prettify("Lorem ipsum\\\ndolor sit");
    assert_eq!(output, "Lorem ipsum\\\ndolor sit");
}

#[test]
fn paragraph_with_inline_html() {
    let output = prettify("Lorem <i>ipsum</i> dolor <s>sit</s>");
    assert_eq!(output, "Lorem <i>ipsum</i> dolor <s>sit</s>");
}

#[test]
fn two_simple_paragraphs() {
    let output = prettify("Lorem ipsum\n\nDolor sit");
    assert_eq!(output, "Lorem ipsum\n\nDolor sit");
}

#[test]
fn two_simple_paragraphs_separated_by_rule() {
    let output = prettify("Lorem ipsum\n___\nDolor sit");
    assert_eq!(output, "Lorem ipsum\n\n---\n\nDolor sit");
}

#[test]
fn paragraph_with_emphasis() {
    let output = prettify("Lorem _ipsum_ dolor __sit__");
    assert_eq!(output, "Lorem *ipsum* dolor **sit**");
}

#[test]
fn paragraph_with_inline_code() {
    let output = prettify("Lorem `ipsum` dolor sit");
    assert_eq!(output, "Lorem `ipsum` dolor sit");
}

#[test]
fn blockquote_with_single_line() {
    let output = prettify("> Lorem ipsum");
    assert_eq!(output, "> Lorem ipsum");
}

#[test]
fn blockquote_with_softbreak() {
    let output = prettify("> Lorem ipsum\n> dolor sit");
    assert_eq!(output, "> Lorem ipsum dolor sit");
}

#[test]
fn blockquote_with_hardbreak() {
    let output = prettify("> Lorem ipsum\\\n> dolor sit");
    assert_eq!(output, "> Lorem ipsum\\\n> dolor sit");
}

#[test]
fn blockquote_with_two_paragrahs() {
    let output = prettify("> Lorem ipsum\n>\n> Dolor sit");
    assert_eq!(output, "> Lorem ipsum\n>\n> Dolor sit");
}

#[test]
fn link_without_title() {
    let output = prettify("[link](google.com)");
    assert_eq!(output, "[link](google.com)");
}

#[test]
fn link_with_title() {
    let output = prettify("[link](google.com \"title\")");
    assert_eq!(output, "[link](google.com \"title\")");
}

#[test]
fn image_without_title() {
    let output = prettify("![foo bar](/path/to/train.jpg)");
    assert_eq!(output, "![foo bar](/path/to/train.jpg)");
}

#[test]
fn image_with_title() {
    let output = prettify("![foo bar](/path/to/train.jpg \"title\")");
    assert_eq!(output, "![foo bar](/path/to/train.jpg \"title\")");
}

#[test]
fn empty_headline() {
    let output = prettify("# ");
    assert_eq!(output, "#");
}

#[test]
fn headlines_with_paragraphs() {
    let output = prettify("# Foo\nLorem ipsum\n\nBar\n---\n\nDolor sit amet");
    assert_eq!(output, "# Foo\n\nLorem ipsum\n\n## Bar\n\nDolor sit amet");
}

#[test]
fn list_with_single_paragraphs() {
    let output = prettify("- Foo\n- Bar\n- Baz");
    assert_eq!(output, "- Foo\n\n- Bar\n\n- Baz");
}

#[test]
fn list_with_multiple_paragraphs() {
    let output = prettify("- Foo\n\n  Bar\n- Baz\n- Quux");
    assert_eq!(output, "- Foo\n\n  Bar\n\n- Baz\n\n- Quux");
}

#[test]
fn numbered_list_with_multiple_paragraphs() {
    let output = prettify("1. Foo\n\n   Bar\n2. Baz\n3. Quux");
    assert_eq!(output, "1. Foo\n\n   Bar\n\n2. Baz\n\n3. Quux");
}

#[test]
fn list_with_mixed_paragraphs_and_blockquotes() {
    let output = prettify("- > Foo\n  >\n  > Bar\n- Baz\n- Quux");
    assert_eq!(output, "- > Foo\n  >\n  > Bar\n\n- Baz\n\n- Quux");
}

#[test]
fn nested_lists() {
    let output = prettify("- Foo\n  * Bar\n  * Baz\n\n- Quux\n  1. Lorem\n  2. Ipsum");
    assert_eq!(output, "- Foo\n\n  - Bar\n\n  - Baz\n\n- Quux\n\n  1. Lorem\n\n  2. Ipsum");
}

#[test]
fn nested_lists_with_blockquotes() {
    let output = prettify("- > Foo\n  > * Bar\n  > * Baz\n\n- > Quux\n  > 1. Lorem\n  > 2. Ipsum");
    assert_eq!(output, "- > Foo\n  >\n  > - Bar\n  >\n  > - Baz\n\n- > Quux\n  >\n  > 1. Lorem\n  >\n  > 2. Ipsum");
}

#[test]
fn simple_code_block() {
    let output = prettify("```rust\nextern crate prettify_cmark;\n```");
    assert_eq!(output, "```rust\nextern crate prettify_cmark;\n```");
}

#[test]
fn code_block_within_block_quote() {
    let output = prettify("> ```rust\n> extern crate prettify_cmark;\n> ```");
    assert_eq!(output, "> ```rust\n> extern crate prettify_cmark;\n> ```");
}

#[test]
fn code_block_within_list_item() {
    let output = prettify("1. ```rust\n   extern crate prettify_cmark;\n   ```");
    assert_eq!(output, "1. ```rust\n   extern crate prettify_cmark;\n   ```");
}
