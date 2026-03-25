use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};

pub struct ZenypstWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
    main_id: FileId,
    source: Source,
    root: Option<std::path::PathBuf>, // directory of the open file, for imports
}

impl ZenypstWorld {
    pub fn new() -> Self {
        let mut book = FontBook::new();
        let mut fonts = Vec::new();

        // Load bundled typst fonts from typst-assets
        for data in typst_assets::fonts() {
            let bytes = Bytes::new(data);
            for font in Font::iter(bytes) {
                book.push(font.info().clone());
                fonts.push(font);
            }
        }

        let main_id = FileId::new(None, VirtualPath::new("main.typ"));
        let source = Source::new(main_id, String::new());

        Self {
            library: LazyHash::new(Library::default()),
            book: LazyHash::new(book),
            fonts,
            main_id,
            source,
            root: None,
        }
    }

    pub fn set_source(&mut self, content: String, root: Option<std::path::PathBuf>) {
        self.source = Source::new(self.main_id, content);
        self.root = root;
    }
}

impl World for ZenypstWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        self.main_id
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main_id {
            return Ok(self.source.clone());
        }
        // Try to read from filesystem relative to root
        if let Some(root) = &self.root {
            let path = root.join(id.vpath().as_rootless_path());
            if let Ok(text) = std::fs::read_to_string(&path) {
                return Ok(Source::new(id, text));
            }
        }
        Err(FileError::NotFound(id.vpath().as_rootless_path().into()))
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        if let Some(root) = &self.root {
            let path = root.join(id.vpath().as_rootless_path());
            if let Ok(data) = std::fs::read(&path) {
                return Ok(Bytes::new(data));
            }
        }
        Err(FileError::NotFound(id.vpath().as_rootless_path().into()))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        use chrono::Datelike;
        let now = chrono::Local::now();
        let naive = if let Some(hours) = offset {
            (now + chrono::Duration::hours(hours)).naive_local().date()
        } else {
            now.naive_local().date()
        };
        Datetime::from_ymd(naive.year(), naive.month() as u8, naive.day() as u8)
    }
}
