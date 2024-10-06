use std::collections::HashMap;

use crate::Event::*;
use mitex::CommandSpec;
use pulldown_cmark::{
    Alignment, BlockQuoteKind, CodeBlockKind, CowStr, Event, LinkType, Tag, TagEnd,
};
use pulldown_cmark_escape::{
    escape_href, escape_html, escape_html_body_text, FmtWriter, IoWriter, StrWrite,
};

enum TableState {
    Head,
    Body,
}

enum ListState {
    None,
    Enumerate(u64),
    Bullet,
}

struct TypstWriter<'a, I, W> {
    /// Iterator supplying events.
    iter: I,

    /// Writer to write to.
    writer: W,

    /// Whether or not the last write wrote a newline.
    end_newline: bool,

    /// Whether if inside a metadata block (text should not be written)
    in_non_writing_block: bool,

    table_state: TableState,
    table_alignments: Vec<Alignment>,
    table_cell_index: usize,
    list_state: ListState,
    numbers: HashMap<CowStr<'a>, usize>,
}

impl<'a, I, W> TypstWriter<'a, I, W>
where
    I: Iterator<Item = Event<'a>>,
    W: StrWrite,
{
    fn new(iter: I, writer: W) -> Self {
        Self {
            iter,
            writer,
            end_newline: true,
            in_non_writing_block: false,
            table_state: TableState::Head,
            table_alignments: vec![],
            table_cell_index: 0,
            list_state: ListState::None,
            numbers: HashMap::new(),
        }
    }

    /// Writes a new line.
    #[inline]
    fn write_newline(&mut self) -> Result<(), W::Error> {
        self.end_newline = true;
        self.writer.write_str("\n")
    }

    /// Writes a buffer, and tracks whether or not a newline was written.
    #[inline]
    fn write(&mut self, s: &str) -> Result<(), W::Error> {
        self.writer.write_str(s)?;

        if !s.is_empty() {
            self.end_newline = s.ends_with('\n');
        }
        Ok(())
    }

    fn run(mut self) -> Result<(), W::Error> {
        while let Some(event) = self.iter.next() {
            match event {
                Start(tag) => {
                    self.start_tag(tag)?;
                }
                End(tag) => {
                    self.end_tag(tag)?;
                }
                Text(text) => {
                    if !self.in_non_writing_block {
                        self.write(&text)?;
                        self.end_newline = text.ends_with('\n');
                    }
                }
                Code(text) => {
                    todo!()
                }
                InlineMath(text) => {
                    let text = mitex::convert_math(&text, None).unwrap();
                    self.write("$")?;
                    self.write(&text)?;
                    self.write("$")?;
                }
                DisplayMath(text) => {
                    let text = mitex::convert_math(&text, None).unwrap();
                    self.write("$\n")?;
                    self.write(&text)?;
                    self.write("\n$")?;
                }
                Html(html) | InlineHtml(html) => {
                    todo!()
                }
                SoftBreak => {
                    self.write_newline()?;
                }
                HardBreak => {
                    self.write("\\\n")?;
                }
                Rule => {
                    todo!()
                }
                FootnoteReference(name) => {
                    todo!()
                }
                TaskListMarker(true) => {
                    todo!()
                }
                TaskListMarker(false) => {
                    todo!()
                }
            }
        }
        Ok(())
    }

    /// Writes the start of an HTML tag.
    fn start_tag(&mut self, tag: Tag<'a>) -> Result<(), W::Error> {
        match tag {
            Tag::HtmlBlock => Ok(()),
            Tag::Paragraph => {
                if self.end_newline {
                } else {
                    self.write("\n")?;
                }
                Ok(())
            }
            Tag::Heading {
                level,
                id,
                classes,
                attrs,
            } => {
                if self.end_newline {
                } else {
                    self.write("\n")?;
                };
                match level {
                    pulldown_cmark::HeadingLevel::H1 => self.write("= "),
                    pulldown_cmark::HeadingLevel::H2 => self.write("== "),
                    pulldown_cmark::HeadingLevel::H3 => self.write("=== "),
                    pulldown_cmark::HeadingLevel::H4 => self.write("==== "),
                    pulldown_cmark::HeadingLevel::H5 => todo!(),
                    pulldown_cmark::HeadingLevel::H6 => todo!(),
                }?;
                Ok(())
            }
            Tag::Table(alignments) => {
                self.table_alignments = alignments;
                todo!()
            }
            Tag::TableHead => {
                self.table_state = TableState::Head;
                self.table_cell_index = 0;
                todo!()
            }
            Tag::TableRow => {
                self.table_cell_index = 0;
                todo!()
            }
            Tag::TableCell => {
                todo!()
            }
            Tag::BlockQuote(kind) => {
                todo!()
            }
            Tag::CodeBlock(info) => {
                todo!()
            }
            Tag::List(Some(1)) => {
                if !self.end_newline {
                    self.write("\n")?;
                }
                self.list_state = ListState::Enumerate(1);
                Ok(())
            }
            Tag::List(Some(start)) => {
                if !self.end_newline {
                    self.write("\n")?;
                }
                self.list_state = ListState::Enumerate(start);
                Ok(())
            }
            Tag::List(None) => {
                if !self.end_newline {
                    self.write("\n")?;
                }
                self.list_state = ListState::Bullet;
                Ok(())
            }
            Tag::Item => {
                if !self.end_newline {
                    self.write("\n")?;
                }
                match self.list_state {
                    ListState::None => todo!(),
                    ListState::Enumerate(_) => self.write("+ "),
                    ListState::Bullet => self.write("- "),
                }
            }
            Tag::DefinitionList => {
                todo!()
            }
            Tag::DefinitionListTitle => {
                todo!()
            }
            Tag::DefinitionListDefinition => {
                todo!()
            }
            Tag::Emphasis => self.write("#emph["),
            Tag::Strong => self.write("#strong["),
            Tag::Strikethrough => todo!(),
            Tag::Link {
                link_type: LinkType::Email,
                dest_url,
                title,
                id: _,
            } => {
                todo!()
            }
            Tag::Link {
                link_type: _,
                dest_url,
                title,
                id: _,
            } => {
                todo!()
            }
            Tag::Image {
                link_type: _,
                dest_url,
                title,
                id: _,
            } => {
                if self.end_newline {
                } else {
                    self.write("\n")?;
                };
                // self.write("\\begin{figure}\n")?;
                self.write("\n")?;
                self.write(&format!("#image(\"{dest_url}\")"))?;
                self.write("\n")?;
                // self.write("\\end{figure}\n")?;
                Ok(())
            }
            Tag::FootnoteDefinition(name) => {
                todo!()
            }
            Tag::MetadataBlock(_) => {
                self.in_non_writing_block = true;
                Ok(())
            }
        }
    }

    fn end_tag(&mut self, tag: TagEnd) -> Result<(), W::Error> {
        match tag {
            TagEnd::HtmlBlock => {}
            TagEnd::Paragraph => {
                self.write("\n")?;
            }
            TagEnd::Heading(level) => {}
            TagEnd::Table => {
                todo!()
            }
            TagEnd::TableHead => {
                todo!()
            }
            TagEnd::TableRow => {
                todo!()
            }
            TagEnd::TableCell => {
                todo!()
            }
            TagEnd::BlockQuote(_) => {
                todo!()
            }
            TagEnd::CodeBlock => {
                todo!()
            }
            TagEnd::List(true) => {
                self.write("")?;
            }
            TagEnd::List(false) => {
                self.write("")?;
            }
            TagEnd::Item => {}
            TagEnd::DefinitionList => {
                todo!()
            }
            TagEnd::DefinitionListTitle => {
                todo!()
            }
            TagEnd::DefinitionListDefinition => {
                todo!()
            }
            TagEnd::Emphasis => {
                self.write("]")?;
            }
            TagEnd::Strong => {
                self.write("]")?;
            }
            TagEnd::Strikethrough => {
                todo!()
            }
            TagEnd::Link => {
                todo!()
            }
            TagEnd::Image => (), // shouldn't happen, handled in start
            TagEnd::FootnoteDefinition => {
                todo!()
            }
            TagEnd::MetadataBlock(_) => {
                self.in_non_writing_block = false;
            }
        }
        Ok(())
    }

    // run raw text, consuming end tag
    fn raw_text(&mut self) -> Result<(), W::Error> {
        let mut nest = 0;
        while let Some(event) = self.iter.next() {
            match event {
                Start(_) => nest += 1,
                End(_) => {
                    if nest == 0 {
                        break;
                    }
                    nest -= 1;
                }
                Html(_) => {}
                InlineHtml(text) | Code(text) | Text(text) => {
                    todo!()
                }
                InlineMath(text) => {
                    self.write("$")?;
                    self.write(&text)?;
                    self.write("$")?;
                }
                DisplayMath(text) => {
                    self.write("\\begin{math}")?;
                    self.write(&text)?;
                    self.write("\\end{math}")?;
                }
                SoftBreak | HardBreak | Rule => {
                    self.write(" ")?;
                }
                FootnoteReference(name) => {
                    todo!()
                }
                TaskListMarker(true) => todo!(),
                TaskListMarker(false) => todo!(),
            }
        }
        Ok(())
    }
}

pub fn push_typst<'a, I>(s: &mut String, iter: I)
where
    I: Iterator<Item = Event<'a>>,
{
    write_typst_fmt(s, iter).unwrap()
}

pub fn write_typst_io<'a, I, W>(writer: W, iter: I) -> std::io::Result<()>
where
    I: Iterator<Item = Event<'a>>,
    W: std::io::Write,
{
    TypstWriter::new(iter, IoWriter(writer)).run()
}

pub fn write_typst_fmt<'a, I, W>(writer: W, iter: I) -> std::fmt::Result
where
    I: Iterator<Item = Event<'a>>,
    W: std::fmt::Write,
{
    TypstWriter::new(iter, FmtWriter(writer)).run()
}
