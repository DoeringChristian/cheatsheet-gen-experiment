use std::collections::HashMap;

use crate::Event::*;
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

struct LatexWriter<'a, I, W> {
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
    numbers: HashMap<CowStr<'a>, usize>,
}

impl<'a, I, W> LatexWriter<'a, I, W>
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
                    self.write("$")?;
                    self.write(&text)?;
                    self.write("$")?;
                }
                DisplayMath(text) => {
                    self.write("\\begin{math}")?;
                    self.write(&text)?;
                    self.write("\\end{math}")?;
                }
                Html(html) | InlineHtml(html) => {
                    todo!()
                }
                SoftBreak => {
                    self.write_newline()?;
                }
                HardBreak => {
                    self.write("\\newline\n")?;
                }
                Rule => {
                    todo!()
                }
                FootnoteReference(name) => {
                    todo!()
                }
                TaskListMarker(true) => {
                    self.write("<input disabled=\"\" type=\"checkbox\" checked=\"\"/>\n")?;
                }
                TaskListMarker(false) => {
                    self.write("<input disabled=\"\" type=\"checkbox\"/>\n")?;
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
                    pulldown_cmark::HeadingLevel::H1 => self.write(r#"\section{"#),
                    pulldown_cmark::HeadingLevel::H2 => self.write(r#"\subsection{"#),
                    pulldown_cmark::HeadingLevel::H3 => todo!(),
                    pulldown_cmark::HeadingLevel::H4 => todo!(),
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
                self.write(r#"\begin{enumerate}"#)
            }
            Tag::List(Some(start)) => todo!(),
            Tag::List(None) => {
                if !self.end_newline {
                    self.write("\n")?;
                }
                self.write(r#"\begin{itemize}"#)
            }
            Tag::Item => {
                if !self.end_newline {
                    self.write("\n")?;
                }
                self.write(r#"\item "#)
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
            Tag::Emphasis => self.write(r#"\emph{"#),
            Tag::Strong => self.write(r#"\textbf{"#),
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
                self.write(&format!(
                    "\\includegraphics[width=0.8\\columnwidth]{{ {dest_url} }}\n"
                ))?;
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
            TagEnd::Heading(level) => {
                match level {
                    pulldown_cmark::HeadingLevel::H1 => self.write(r#"}"#),
                    pulldown_cmark::HeadingLevel::H2 => self.write(r#"}"#),
                    pulldown_cmark::HeadingLevel::H3 => todo!(),
                    pulldown_cmark::HeadingLevel::H4 => todo!(),
                    pulldown_cmark::HeadingLevel::H5 => todo!(),
                    pulldown_cmark::HeadingLevel::H6 => todo!(),
                }?;
            }
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
                self.write("\\end{enumerate}\n")?;
            }
            TagEnd::List(false) => {
                self.write("\\end{itemize}\n")?;
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
                self.write("}}")?;
            }
            TagEnd::Strong => {
                self.write("}}")?;
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
                TaskListMarker(true) => self.write("[x]")?,
                TaskListMarker(false) => self.write("[ ]")?,
            }
        }
        Ok(())
    }
}

pub fn push_latex<'a, I>(s: &mut String, iter: I)
where
    I: Iterator<Item = Event<'a>>,
{
    write_latex_fmt(s, iter).unwrap()
}

pub fn write_latex_io<'a, I, W>(writer: W, iter: I) -> std::io::Result<()>
where
    I: Iterator<Item = Event<'a>>,
    W: std::io::Write,
{
    LatexWriter::new(iter, IoWriter(writer)).run()
}

pub fn write_latex_fmt<'a, I, W>(writer: W, iter: I) -> std::fmt::Result
where
    I: Iterator<Item = Event<'a>>,
    W: std::fmt::Write,
{
    LatexWriter::new(iter, FmtWriter(writer)).run()
}
