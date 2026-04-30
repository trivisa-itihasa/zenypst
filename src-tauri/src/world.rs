use std::path::PathBuf;
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};
use typst_kit::download::{Downloader, ProgressSink};
use typst_kit::package::PackageStorage;

pub struct ZenypstWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
    main_id: FileId,
    source: Source,
    root: Option<PathBuf>, // directory of the open file, for imports
    package_storage: PackageStorage,
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

        // Load system fonts so that user-specified fonts in Typst source work
        let mut db = fontdb::Database::new();
        db.load_system_fonts();
        let mut loaded_paths = std::collections::HashSet::new();
        for face in db.faces() {
            if let fontdb::Source::File(path) = &face.source {
                if loaded_paths.insert(path.clone()) {
                    if let Ok(data) = std::fs::read(path) {
                        let bytes = Bytes::new(data);
                        for font in Font::iter(bytes) {
                            book.push(font.info().clone());
                            fonts.push(font);
                        }
                    }
                }
            }
        }

        let main_id = FileId::new(None, VirtualPath::new("main.typ"));
        let source = Source::new(main_id, String::new());

        let downloader = Downloader::new("zenypst/0.2.3");
        let package_storage = PackageStorage::new(None, None, downloader);

        Self {
            library: LazyHash::new(Library::default()),
            book: LazyHash::new(book),
            fonts,
            main_id,
            source,
            root: None,
            package_storage,
        }
    }

    pub fn set_source(&mut self, content: String, root: Option<PathBuf>) {
        self.source = Source::new(self.main_id, content);
        self.root = root;
    }

    fn resolve_path(&self, id: FileId) -> Option<PathBuf> {
        // Try local filesystem relative to root
        if let Some(root) = &self.root {
            let path = root.join(id.vpath().as_rootless_path());
            if path.exists() {
                return Some(path);
            }
        }

        // Try package cache
        if let Some(spec) = id.package() {
            let progress = &mut ProgressSink;
            if let Ok(package_path) = self.package_storage.prepare_package(spec, progress) {
                let path = package_path.join(id.vpath().as_rootless_path());
                if path.exists() {
                    return Some(path);
                }
            }
        }

        None
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
        if let Some(path) = self.resolve_path(id) {
            if let Ok(text) = std::fs::read_to_string(&path) {
                return Ok(Source::new(id, text));
            }
        }
        Err(FileError::NotFound(id.vpath().as_rootless_path().into()))
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        if let Some(path) = self.resolve_path(id) {
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
