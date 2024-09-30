use std::path::{Path, PathBuf};

use pulldown_cmark as md;
use serde::Serialize;
use tera::{Context, Tera};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    folder: PathBuf,

    #[arg(short, long, default_value = "out")]
    out: PathBuf,
}

#[derive(Serialize, Debug)]
struct Block {
    content: String,
}

fn main() {
    let args = Args::parse();

    let mut md_files = vec![];
    let mut other_paths = vec![];

    args.folder.read_dir().unwrap().for_each(|entry| {
        let entry = entry.unwrap();
        let path = entry.path();
        if let Some(extension) = path.extension() {
            if extension == "md" {
                md_files.push(path);
            } else {
                other_paths.push(path);
            }
        } else {
            if path.file_name().unwrap() != "templates" {
                other_paths.push(path);
            }
        }
    });

    dbg!(&args);

    let blocks = md_files
        .into_iter()
        .map(|filename| {
            dbg!(&filename);
            let src = std::fs::read_to_string(filename).unwrap();

            let mut options = md::Options::empty();
            options.insert(md::Options::ENABLE_MATH);
            options.insert(md::Options::ENABLE_TABLES);

            let parser = md::Parser::new(&src);

            let mut html = String::new();
            pulldown_cmark::html::push_html(&mut html, parser);

            Block { content: html }
        })
        .collect::<Vec<_>>();

    fs_extra::copy_items(
        &other_paths,
        args.out.to_str().unwrap(),
        &fs_extra::dir::CopyOptions::default().overwrite(true),
    )
    .unwrap();

    let mut tera = Tera::new(args.folder.join("templates/*.html").to_str().unwrap()).unwrap();
    tera.autoescape_on(vec![]);

    let mut context = Context::new();
    context.insert("blocks", &blocks);

    let result = tera.render("index.html", &context).unwrap();

    dbg!(&result);

    std::fs::write(args.out.join("index.html"), result).unwrap();

    // dbg!(&ast);
}
