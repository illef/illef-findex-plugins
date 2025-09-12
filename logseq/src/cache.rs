use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use shellexpand::tilde;
use thiserror::Error;
use ureq::serde_json;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LogseqPage {
    pub title: String,
    pub uuid: String,
    pub tags: Vec<String>,
    pub updated_at: Option<i64>,
}

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
}

pub struct FilePageCache {
    file_name: PathBuf,
}

impl FilePageCache {
    pub fn default() -> FilePageCache {
        return FilePageCache::new(&*tilde(&format!(
            "~/.cache/illef-findex-plugin/logseq.pages.cache.json",
        )));
    }

    pub fn new<P: AsRef<Path>>(file_name: P) -> Self {
        FilePageCache {
            file_name: file_name.as_ref().to_path_buf(),
        }
    }

    pub fn update_cache(&self, pages: Vec<LogseqPage>) -> Result<(), CacheError> {
        if let Some(parent_dir) = self.file_name.parent() {
            std::fs::create_dir_all(parent_dir)?;
        }

        let json_str = serde_json::to_string(&pages)?;
        std::fs::write(&self.file_name, json_str)?;

        Ok(())
    }

    pub fn load_cache(&self) -> Result<Vec<LogseqPage>, CacheError> {
        let file_contents = std::fs::read_to_string(&self.file_name)?;
        let pages: Vec<LogseqPage> = serde_json::from_str(&file_contents)?;
        Ok(pages)
    }
}
