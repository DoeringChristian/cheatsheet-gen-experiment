use std::path::{Path, PathBuf};

use itertools::Itertools;
use pdf_writer::Finish;
use pulldown_cmark::{self as md, HeadingLevel, LinkType, Tag};
use serde::Serialize;
use tera::{Context, Tera};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    input: Vec<PathBuf>,

    #[arg(short, long, default_value = "data/templates/")]
    templates: PathBuf,

    #[arg(short, long, default_value = "out")]
    out: PathBuf,
}

#[derive(Serialize, Debug)]
struct Block {
    content: String,
}

fn main() {
    let args = Args::parse();

    dbg!(&args);

    let mut additional_files = vec![];

    let blocks = args
        .input
        .into_iter()
        .flat_map(|filename| {
            dbg!(&filename);
            let src = std::fs::read_to_string(&filename).unwrap();

            let mut options = md::Options::empty();
            options.insert(md::Options::ENABLE_MATH);
            options.insert(md::Options::ENABLE_TABLES);
            options.insert(md::Options::ENABLE_GFM);
            options.insert(md::Options::ENABLE_FOOTNOTES);
            options.insert(md::Options::ENABLE_STRIKETHROUGH);
            options.insert(md::Options::ENABLE_SMART_PUNCTUATION);
            options.insert(md::Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);
            options.insert(md::Options::ENABLE_DEFINITION_LIST);

            let parser = md::Parser::new_ext(&src, options);

            // Preprocessing
            let parser = parser.map(|event| match event {
                // pulldown_cmark::Event::Start(Tag::BlockQuote(Some(kind))) => md::Event::Html(),
                _ => event,
            });

            // Pull in aditional files
            let parser = parser.map(|event| {
                dbg!(&event);
                match &event {
                    pulldown_cmark::Event::Start(Tag::Link { .. }) => todo!(),
                    pulldown_cmark::Event::Start(Tag::Image {
                        link_type: LinkType::Inline,
                        dest_url,
                        ..
                    }) => additional_files.push(
                        filename
                            .parent()
                            .unwrap()
                            .join(dest_url.as_ref())
                            .to_path_buf(),
                    ),
                    _ => {}
                };
                event
            });

            // Post Processing
            let parser = parser.map(|event| match event {
                _ => event,
            });

            // Split by H1 headings
            let mut chunk_id = 0;
            let parsers = parser.chunk_by(|event| match event {
                pulldown_cmark::Event::Start(Tag::Heading {
                    level: HeadingLevel::H1,
                    ..
                }) => {
                    chunk_id += 1;
                    chunk_id
                }
                _ => chunk_id,
            });

            // Render HTML and generate blocks
            let blocks = parsers
                .into_iter()
                .map(|(i, parser)| {
                    let mut html = String::new();
                    pulldown_cmark::html::push_html(&mut html, parser);
                    Block { content: html }
                })
                .collect::<Vec<_>>();

            blocks
        })
        .collect::<Vec<_>>();

    dbg!(&additional_files);
    fs_extra::copy_items(
        &additional_files,
        args.out.to_str().unwrap(),
        &fs_extra::dir::CopyOptions::default().overwrite(true),
    )
    .unwrap();

    fs_extra::copy_items(
        &[&args.templates.join("css")],
        args.out.to_str().unwrap(),
        &fs_extra::dir::CopyOptions::default().overwrite(true),
    )
    .unwrap();

    let mut tera = Tera::new(args.templates.join("*.html").to_str().unwrap()).unwrap();
    tera.autoescape_on(vec![]);

    let mut context = Context::new();
    context.insert("blocks", &blocks);

    let result = tera.render("index.html", &context).unwrap();

    dbg!(&result);

    std::fs::write(args.out.join("index.html"), result).unwrap();
}
