use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use shellexpand::tilde;
use thiserror::Error;
use ureq::serde_json;

use super::types::Item;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
}

pub struct FileItemCache {
    file_name: PathBuf,
}

impl FileItemCache {
    pub fn default() -> FileItemCache {
        return FileItemCache::new(&*tilde(&format!(
            "~/.cache/illef-findex-plugin/raindrop.cache.json",
        )));
    }

    pub fn new<P: AsRef<Path>>(file_name: P) -> Self {
        FileItemCache {
            file_name: file_name.as_ref().to_path_buf(),
        }
    }

    pub fn update_cache(&self, items: Vec<Item>) -> Result<(), CacheError> {
        let json_str = serde_json::to_string(&items)?;
        std::fs::write(&self.file_name, json_str)?;

        Ok(())
    }

    pub fn load_cache(&self) -> Result<Vec<Item>, CacheError> {
        let file_contents = std::fs::read_to_string(&self.file_name)?;
        let items: Vec<Item> = serde_json::from_str(&file_contents)?;
        Ok(items)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ItemScore {
    pub score: i32,
    pub last_accessed_time: u64,
    pub id: i32,
}

pub struct FileItemScoreCache {
    file_name: PathBuf,
}

impl FileItemScoreCache {
    pub fn default() -> Self {
        return Self::new(&*tilde(&format!(
            "~/.cache/illef-findex-plugin/raindrop.score.cache.json",
        )));
    }

    pub fn new<P: AsRef<Path>>(file_name: P) -> Self {
        FileItemScoreCache {
            file_name: file_name.as_ref().to_path_buf(),
        }
    }

    pub fn update_item_scores(&self) {
        let mut item_scores = self.load_item_scores();
        let access_log_file =
            tilde(&format!("~/.cache/illef-findex-plugin/access_log")).to_string();

        if let Ok(file_contents) = std::fs::read_to_string(&access_log_file) {
            for line in file_contents.lines() {
                let elements: Vec<_> = line.split("|").map(|l| l.trim()).take(2).collect();
                if let (Some(id), Some(access_time)) = (elements.get(0), elements.get(1)) {
                    if let (Ok(id), Ok(last_accessed_time)) =
                        (id.parse::<i32>(), access_time.parse::<u64>())
                    {
                        if let Some(item_score) = item_scores.iter_mut().find(|i| i.id == id) {
                            item_score.score += 1;
                            item_score.last_accessed_time = last_accessed_time;
                        } else {
                            item_scores.push(ItemScore {
                                score: 1,
                                id,
                                last_accessed_time,
                            });
                        }
                    }
                }
            }
            let json_str = serde_json::to_string(&item_scores).expect("item to json");
            std::fs::write(&self.file_name, json_str).expect("file write fail");
            std::fs::remove_file(&access_log_file).expect("remove access file");
        }
    }

    pub fn load_item_scores(&self) -> Vec<ItemScore> {
        if let Ok(file_contents) = std::fs::read_to_string(&self.file_name) {
            if let Ok(mut items) = serde_json::from_str::<Vec<ItemScore>>(&file_contents) {
                items.sort_by(|a, b| b.last_accessed_time.cmp(&a.last_accessed_time));
                return items;
            }
        }
        vec![]
    }
}

// write test
#[cfg(test)]
mod tests {
    use super::*;
    use fake::{Fake, Faker};
    use tempfile::Builder;

    #[test]
    fn test_write_load_cache() {
        let items: Vec<Item> = Faker.fake();
        let temp_file_path = Builder::new().tempfile().unwrap();
        let cache = FileItemCache::new(temp_file_path);
        cache.update_cache(items.clone()).unwrap();
        let loaded_cache = cache.load_cache().unwrap();

        assert_eq!(items, loaded_cache);
    }
}
