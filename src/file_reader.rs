mod basic;
mod cached;

pub trait FileReader {
    fn read_file(&self, path: &str) -> Result<Vec<u8>, std::io::Error>;
}

pub use cached::CachedFileReader;

