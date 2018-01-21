use std::fmt::{Result, Write};

use pulldown_cmark::{Event, Tag};

use writer::{Frame, Writer};

/// Event-driven pretty printer for CommonMark documents.
///
/// The printer can be driven by pushing events into it, which can be obtained
/// using `pulldown_cmark::Parser`.
///
/// # Examples
///
/// ```rust
/// extern crate pulldown_cmark;
/// extern crate prettify_cmark;
///
/// use pulldown_cmark::Parser;
/// use prettify_cmark::PrettyPrinter;
///
/// fn main() {
///     let events = Parser::new("Lorem _ipsum_!\n\nDolor `sit`.");
///     let mut printer = PrettyPrinter::new_with_prefix(String::new(), "///");
///     printer.push_events(events).unwrap();
///
///     assert_eq!(printer.into_inner(), "/// Lorem *ipsum*!\n///\n/// Dolor `sit`.")
/// }
/// ```
pub struct PrettyPrinter<W = String> {
    writer: Writer<W>,
    needs_break: bool
}

impl<W: Write> PrettyPrinter<W> {
    /// Create a new pretty printer that wraps around a writer.
    pub fn new(write: W) -> PrettyPrinter<W> {
        PrettyPrinter::new_with_prefix(write, "")
    }

    /// Create a new pretty printer with a prefix that wraps around
    /// a writer.
    ///
    /// The prefix will be applied to all lines that are produced by
    /// the printer.
    pub fn new_with_prefix(write: W, prefix: &str) -> PrettyPrinter<W> {
        PrettyPrinter {
            writer: Writer::new(write, prefix.to_string()),
            needs_break: false
        }
    }

    /// Push a single event into the printer.
    ///
    /// Events can be obtained using `pulldown_cmark::Parser`.
    pub fn push_event<'a>(&mut self, event: Event<'a>) -> Result {
        match event {
            Event::Start(tag) => {
                match tag {
                    Tag::Paragraph => {
                        self.flush_break()?;
                    },
                    Tag::Rule => {
                        self.flush_break()?;
                        self.writer.write_text("---")?;
                    },
                    Tag::Header(indent) => {
                        self.flush_break()?;
                        self.writer.write_text(&"#".repeat(indent as usize))?;
                        self.writer.write_non_breaking_space()?;
                    },
                    Tag::List(start) => {
                        self.flush_break()?;
                        self.writer.push_frame(Frame::ListItem(start));
                    },
                    Tag::Item => {
                        match self.writer.pop_frame() {
                            Some(Frame::ListItem(None)) => {
                                self.flush_break()?;
                                self.writer.write_text("-")?;
                                self.writer.write_non_breaking_space()?;
                                self.writer.push_frame(Frame::ListItem(None));
                            },
                            Some(Frame::ListItem(Some(index))) => {
                                self.flush_break()?;
                                write!(self.writer, "{}.", index)?;
                                self.writer.write_non_breaking_space()?;
                                self.writer.push_frame(Frame::ListItem(Some(index + 1)));
                            },
                            _ => {}
                        }
                    },
                    Tag::BlockQuote => {
                        self.flush_break()?;
                        self.writer.write_text(">")?;
                        self.writer.write_non_breaking_space()?;
                        self.writer.push_frame(Frame::BlockQuote);
                    },
                    Tag::CodeBlock(note) => {
                        self.flush_break()?;
                        write!(self.writer, "```{}", note)?;
                        self.writer.write_hard_break()?;
                        self.writer.write_indent()?;
                    },
                    Tag::Emphasis => {
                        self.writer.write_text("*")?;
                    },
                    Tag::Strong => {
                        self.writer.write_text("**")?;
                    },
                    Tag::Code => {
                        self.writer.write_text("`")?;
                    },
                    Tag::Link(_, _) => {
                        self.writer.write_text("[")?;
                    },
                    Tag::Image(_, _) => {
                        self.writer.write_text("![")?;
                    },
                    Tag::FootnoteDefinition(_) => { /* not supported for now */ },
                    Tag::Table(_) => { /* not supported for now */ },
                    Tag::TableHead => { /* not supported for now */ },
                    Tag::TableRow => { /* not supported for now */ },
                    Tag::TableCell => { /* not supported for now */ }
                }
            },
            Event::End(tag) => {
                match tag {
                    Tag::Paragraph => {
                        self.needs_break = true;
                    },
                    Tag::Rule => {
                        self.needs_break = true;
                    },
                    Tag::Header(_) => {
                        self.needs_break = true;
                    },
                    Tag::List(_) => {
                        self.writer.pop_frame();
                        self.needs_break = true;
                    },
                    Tag::Item => {
                        self.needs_break = true;
                    },
                    Tag::BlockQuote => {
                        self.writer.pop_frame();
                        self.needs_break = true;
                    },
                    Tag::CodeBlock(_) => {
                        self.writer.write_text("```")?;
                        self.needs_break = true;
                    },
                    Tag::Emphasis => {
                        self.writer.write_text("*")?;
                    },
                    Tag::Strong => {
                        self.writer.write_text("**")?;
                    },
                    Tag::Code => {
                        self.writer.write_text("`")?;
                    },
                    Tag::Link(ref url, ref title) | Tag::Image(ref url, ref title) => {
                        if title.is_empty() {
                            write!(self.writer, "]({})", url)?;
                        } else {
                            write!(self.writer, "]({} \"{}\")", url, title)?;
                        }
                    },
                    Tag::FootnoteDefinition(_) => { /* not supported for now */ },
                    Tag::Table(_) => { /* not supported for now */ },
                    Tag::TableHead => { /* not supported for now */ },
                    Tag::TableRow => { /* not supported for now */ },
                    Tag::TableCell => { /* not supported for now */ }
                }
            },
            Event::Text(text) => {
                for (i, line) in text.split('\n').enumerate() {
                    if i > 0 {
                        self.writer.write_hard_break()?;
                        self.writer.write_indent()?;
                    }
                    self.writer.write_text(line)?;
                }
            },
            Event::Html(_html) => {
                // not supported for now
            },
            Event::InlineHtml(html) => {
                self.writer.write_text(html.as_ref())?
            },
            Event::FootnoteReference(_) => {
                // not supported for now
            },
            Event::SoftBreak => {
                self.writer.write_soft_break()?
            },
            Event::HardBreak => {
                self.writer.write_text("\\")?;
                self.writer.write_hard_break()?;
                self.writer.write_indent()?;
            }
        };

        Ok(())
    }

    /// Push a series of events into the printer.
    ///
    /// Events can be obtained using `pulldown_cmark::Parser`.
    pub fn push_events<'a, I>(&mut self, events: I) -> Result
        where I: IntoIterator<Item=Event<'a>>
    {
        for event in events {
            self.push_event(event)?;
        }
        Ok(())
    }

    /// Unwrap the printer, returning the underlying writer.
    pub fn into_inner(self) -> W {
        self.writer.into_inner()
    }

    fn flush_break(&mut self) -> Result {
        if self.needs_break {
            self.writer.write_hard_break()?;
            self.writer.write_indent()?;
            self.writer.write_hard_break()?;
            self.writer.write_indent()?;
        }
        self.needs_break = false;
        Ok(())
    }
}

impl Default for PrettyPrinter {
    fn default() -> PrettyPrinter {
        PrettyPrinter::new(String::new())
    }
}
