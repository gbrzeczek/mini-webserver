use crate::file_reader::basic::BasicFileReader;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::io::Error;
use std::sync::{Arc, RwLock};

use super::FileReader;

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
