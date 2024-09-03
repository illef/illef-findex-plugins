use convert_case::{Case, Casing};
use html_parser::{Dom, Node};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader},
    path::Path,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoaderError {
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
}

pub struct ZoteroItem {
    pub title: String,
    pub tags: Vec<String>,
    pub select: String,
    pub icon: String,
    pub date_modified: String,
}

fn get_text_from_node(node: &Node) -> Option<String> {
    match node {
        Node::Text(text) => return Some(text.clone()),
        Node::Element(element) => {
            for node in element.children.iter() {
                let text = get_text_from_node(&node);
                if text.is_some() {
                    return text;
                }
            }
            return None;
        }
        _ => return None,
    }
}

fn extract_first_tag_content(input: &str) -> Option<String> {
    if let Ok(dom) = Dom::parse(input) {
        for node in dom.children.iter() {
            if let Some(text) = get_text_from_node(node) {
                return Some(text);
            }
        }
    }
    return None;
}

fn item_type_to_icon(item_type: &str) -> String {
    item_type.to_case(Case::Kebab)
}

impl Item {
    fn into(self) -> Vec<ZoteroItem> {
        // TODO: self.note 도 꺼내서 별도 Item 으로 추가해야 한다
        let tags = self.tags.into_iter().map(|t| t.tag).collect();

        let title: String = if self.item_type == "note" {
            self.note
                .map(|n| extract_first_tag_content(&n))
                .flatten()
                .unwrap_or("<Unknown Title>".into())
        } else {
            self.short_title
                .or(self.title)
                .unwrap_or("<Unknown Title>".into())
        };

        let title = if let Some(publication_title) = self.publication_title {
            format!("{} | {} | {}", title, publication_title, self.item_type)
        } else {
            format!("{} | {}", title, self.item_type)
        };

        let date_modified = self
            .notes
            .map(|n| {
                n.into_iter()
                    .max_by(|a, b| a.date_modified.cmp(&b.date_modified))
            })
            .flatten()
            .map(|n| std::cmp::max(n.date_modified, self.date_modified.clone()))
            .unwrap_or(self.date_modified.clone());

        vec![ZoteroItem {
            title,
            tags,
            select: self.select,
            icon: item_type_to_icon(&self.item_type),
            date_modified,
        }]
    }
}

pub struct BibTexLoader {}

impl BibTexLoader {
    pub fn load_zotero<P: AsRef<Path>>(file_name: P) -> Result<ZoteroData, LoaderError> {
        let file = File::open(file_name)?;
        let reader = BufReader::new(file);
        let data: ZoteroData = serde_json::from_reader(reader)?;
        Ok(data)
    }

    pub fn load_items(data: ZoteroData) -> Vec<ZoteroItem> {
        data.items.into_iter().map(|i| i.into()).flatten().collect()
    }
}

// write test
#[cfg(test)]
mod tests {
    use super::*;
    use shellexpand::tilde;

    #[test]
    fn test_load_zotero() {
        let data = BibTexLoader::load_zotero(&*tilde("~/.cache/zotero/My-Library.json"));
        assert!(data.is_ok());
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ZoteroData {
    config: Config,
    collections: HashMap<String, Collection>,
    items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    id: String,
    label: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Collection {
    key: String,
    parent: String,
    name: String,
    // collections: Vec<String>, TODO: Inner Collection
    items: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    key: String,
    #[serde(rename = "itemType")]
    item_type: String,
    title: Option<String>,
    note: Option<String>,
    #[serde(rename = "shortTitle")]
    short_title: Option<String>,
    date: Option<String>,
    language: Option<String>,
    #[serde(rename = "libraryCatalog")]
    library_catalog: Option<String>,
    #[serde(rename = "accessDate")]
    access_date: Option<String>,
    tags: Vec<Tag>,
    collections: Option<Vec<String>>,
    #[serde(rename = "publicationTitle")]
    publication_title: Option<String>,
    #[serde(rename = "dateAdded")]
    date_added: String,
    #[serde(rename = "dateModified")]
    date_modified: String,
    extra: Option<String>,
    select: String,
    notes: Option<Vec<Note>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tag {
    tag: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Note {
    #[serde(rename = "itemType")]
    item_type: String,
    #[serde(rename = "parentItem")]
    parent_item: String,
    note: String,
    tags: Vec<Tag>,
    #[serde(rename = "dateAdded")]
    date_added: String,
    #[serde(rename = "dateModified")]
    date_modified: String,
}
