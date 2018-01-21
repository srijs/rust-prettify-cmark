use std::fmt::{Result, Write};

#[derive(Debug, PartialEq, Eq)]
pub enum Frame {
    ListItem(Option<usize>),
    BlockQuote
}

struct Output<W> {
    inner: W,
    needs_space: usize
}

impl<W: Write> Output<W> {
    pub fn write_text(&mut self, text: &str) -> Result {
        if self.needs_space > 0 {
            let space = " ".repeat(self.needs_space);
            self.inner.write_str(&space)?;
            self.needs_space = 0;
        }
        self.inner.write_str(text)
    }

    pub fn write_hard_break(&mut self) -> Result {
        self.needs_space = 0;
        self.inner.write_str("\n")
    }

    pub fn write_soft_break(&mut self) -> Result {
        self.needs_space = 0;
        // we'll deal with line wrapping later
        self.inner.write_str(" ")
    }
}

pub struct Writer<W> {
    prefix: String,
    frames: Vec<Frame>,
    output: Output<W>
}

impl<W: Write> Writer<W> {
    pub fn new(output: W, prefix: String) -> Writer<W> {
        Writer {
            prefix: prefix,
            frames: vec![],
            output: Output { inner: output, needs_space: 0 }
        }
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    pub fn pop_frame(&mut self) -> Option<Frame> {
        self.frames.pop()
    }

    pub fn write_text(&mut self, text: &str) -> Result {
        self.output.write_text(text)
    }

    pub fn write_hard_break(&mut self) -> Result {
        self.output.write_hard_break()
    }

    pub fn write_soft_break(&mut self) -> Result {
        self.output.write_soft_break()
    }

    pub fn write_non_breaking_space(&mut self) -> Result {
        self.output.needs_space += 1;
        Ok(())
    }

    pub fn write_indent(&mut self) -> Result {
        self.output.inner.write_str(&self.prefix)?;
        for frame in &self.frames[..] {
            match frame {
                &Frame::ListItem(None) => {
                    self.output.needs_space += 2;
                },
                &Frame::ListItem(Some(index)) => {
                    let indent = (index / 10) + 3;
                    self.output.needs_space += indent;
                },
                &Frame::BlockQuote => {
                    self.output.write_text(">")?;
                    self.output.needs_space += 1;
                }
            }
        }
        Ok(())
    }

    pub fn into_inner(self) -> W {
        self.output.inner
    }
}

impl<W: Write> Write for Writer<W> {
    fn write_str(&mut self, s: &str) -> Result {
        self.output.write_text(s)
    }
}
