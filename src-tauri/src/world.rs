use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex as StdMutex;
use std::time::SystemTime;
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
    source_cache: StdMutex<HashMap<FileId, (Source, SystemTime)>>,
    file_cache: StdMutex<HashMap<FileId, (Bytes, SystemTime)>>,
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
            source_cache: StdMutex::new(HashMap::new()),
            file_cache: StdMutex::new(HashMap::new()),
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
            if let Ok(meta) = std::fs::metadata(&path) {
                if let Ok(mtime) = meta.modified() {
                    let cache = self.source_cache.lock().unwrap();
                    if let Some((cached_src, cached_mtime)) = cache.get(&id) {
                        if *cached_mtime == mtime {
                            return Ok(cached_src.clone());
                        }
                    }
                    drop(cache);
                }
            }
            if let Ok(text) = std::fs::read_to_string(&path) {
                let src = Source::new(id, text);
                if let Ok(meta) = std::fs::metadata(&path) {
                    if let Ok(mtime) = meta.modified() {
                        let mut cache = self.source_cache.lock().unwrap();
                        cache.insert(id, (src.clone(), mtime));
                    }
                }
                return Ok(src);
            }
        }
        Err(FileError::NotFound(id.vpath().as_rootless_path().into()))
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        if let Some(path) = self.resolve_path(id) {
            if let Ok(meta) = std::fs::metadata(&path) {
                if let Ok(mtime) = meta.modified() {
                    let cache = self.file_cache.lock().unwrap();
                    if let Some((cached_bytes, cached_mtime)) = cache.get(&id) {
                        if *cached_mtime == mtime {
                            return Ok(cached_bytes.clone());
                        }
                    }
                    drop(cache);
                }
            }
            if let Ok(data) = std::fs::read(&path) {
                let bytes = Bytes::new(data);
                if let Ok(meta) = std::fs::metadata(&path) {
                    if let Ok(mtime) = meta.modified() {
                        let mut cache = self.file_cache.lock().unwrap();
                        cache.insert(id, (bytes.clone(), mtime));
                    }
                }
                return Ok(bytes);
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
