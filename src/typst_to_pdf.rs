use std::path::{Path, PathBuf};

use chrono::{DateTime, Datelike, Local};
use comemo::Prehashed;
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook, FontFamily};
use typst::utils::LazyHash;
use typst::{Library, World};
use typst_kit::fonts::{FontSlot, Fonts};
use typst_pdf::PdfOptions;

const LIBRARIES: &[(&str, &str)] = &[];

pub struct TypstWorld {
    library: LazyHash<Library>,
    font_book: LazyHash<FontBook>,
    fonts: Vec<FontSlot>,
    source: Source,
    date: DateTime<Local>,
    ext_libraries: Vec<Source>,
    outdir: PathBuf,
    additional: Vec<PathBuf>,
}

impl TypstWorld {
    pub fn new(
        source: String,
        ext_libraries: &[(&str, &str)],
        outdir: &Path,
        additional: Vec<PathBuf>,
    ) -> Self {
        // let fonts: Vec<Font> = fonts
        //     .iter()
        //     .map(|f| Font::new(Bytes::from(*f), 0).unwrap())
        //     .collect();
        let fonts = Fonts::searcher()
            .include_embedded_fonts(true)
            .include_system_fonts(false)
            .search();

        let ext_libraries: Vec<Source> = ext_libraries
            .iter()
            .map(|(p, f)| Source::new(FileId::new(None, VirtualPath::new(p)), String::from(*f)))
            .collect();

        let library = Library::builder().build();

        Self {
            library: LazyHash::new(library),
            font_book: LazyHash::new(fonts.book),
            fonts: fonts.fonts,
            source: Source::new(FileId::new(None, VirtualPath::new("/main.typ")), source),
            date: Local::now(),
            ext_libraries,
            outdir: outdir.to_path_buf(),
            additional,
        }
    }
}

impl World for TypstWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.font_book
    }

    fn main(&self) -> FileId {
        self.source.id()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.source.id() {
            Ok(self.source.clone())
        } else if self.ext_libraries.iter().find(|l| id == l.id()).is_some() {
            Ok(self
                .ext_libraries
                .iter()
                .find(|l| id == l.id())
                .unwrap()
                .clone())
        } else {
            Err(FileError::NotFound(
                id.vpath().as_rooted_path().to_path_buf(),
            ))
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        let result = std::fs::read(self.outdir.join(id.vpath().as_rootless_path())).unwrap();
        Ok(result.as_slice().into())
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts[index].get()
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        let date = if let Some(offset) = offset {
            self.date.naive_utc() + chrono::Duration::try_hours(offset)?
        } else {
            self.date.naive_local()
        };

        Datetime::from_ymd(
            date.year(),
            date.month().try_into().ok()?,
            date.day().try_into().ok()?,
        )
    }
}

pub fn to_pdf(source: String, outdir: &Path, additional: Vec<PathBuf>) -> Vec<u8> {
    dbg!(&additional);
    let world = TypstWorld::new(source, LIBRARIES, outdir, additional);

    let doc = typst::compile(&world).output.unwrap();
    let pdf = typst_pdf::pdf(&doc, &PdfOptions::default()).unwrap();

    return pdf;
}
