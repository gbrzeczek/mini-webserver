use crate::config::Config;
use std::io::{Error, ErrorKind};
use std::fs;

use super::FileReader;

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
