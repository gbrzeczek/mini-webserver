use crate::config::Config;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::sync::{Arc, RwLock};
use std::fs;

pub trait FileReader {
    fn read_file(&self, path: &str) -> Result<Vec<u8>, Error>;
}

pub struct BasicFileReader;

impl FileReader for BasicFileReader {
    fn read_file(&self, path: &str) -> Result<Vec<u8>, Error> {
        let project_dir = std::env::current_dir()?;
        let base = project_dir.join(Config::base_path());

        let requested = base.join(path.trim_start_matches("/"));

        if !requested.is_file() {
            return Err(Error::new(ErrorKind::NotFound, "File not found"));
        }

        fs::read(requested)
    }
}

struct CacheEntry {
    contents: Vec<u8>,
}

pub struct CachedFileReader {
    reader: Box<dyn FileReader + Send + Sync>,
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
}

impl CachedFileReader {
    pub fn instance() -> &'static CachedFileReader {
        static INSTANCE: Lazy<CachedFileReader> = Lazy::new(|| CachedFileReader {
            reader: Box::new(BasicFileReader),
            cache: Arc::new(RwLock::new(HashMap::new())),
        });
        &INSTANCE
    }
}

impl FileReader for CachedFileReader {
    fn read_file(&self, path: &str) -> Result<Vec<u8>, Error> {
        if let Ok(cache) = self.cache.read() {
            if let Some(entry) = cache.get(path) {
                return Ok(entry.contents.clone());
            }
        }

        let content = self.reader.read_file(path)?;

        if let Ok(mut cache) = self.cache.write() {
            cache.insert(
                path.to_string(),
                CacheEntry {
                    contents: content.clone(),
                },
            );
        }

        Ok(content)
    }
}
