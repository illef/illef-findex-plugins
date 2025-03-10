use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RainDropResponse {
    pub result: bool,
    pub items: Vec<Item>,
    pub count: u32,
    pub collection_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Dummy, Clone, PartialEq)]
pub struct Item {
    #[serde(rename = "_id")]
    pub id: i32,
    pub link: String,
    pub title: String,
    pub excerpt: String,
    pub note: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub user: UserRef,
    pub cover: String,
    pub media: Vec<Media>,
    pub tags: Vec<String>,
    pub important: Option<bool>,
    pub removed: bool,
    pub created: String,
    pub collection: CollectionRef,
    // pub highlights: Vec<String>,
    #[serde(rename = "lastUpdate")]
    pub last_update: String,
    pub domain: String,
    pub sort: i32,
    #[serde(rename = "collectionId")]
    pub collection_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Dummy, Clone, PartialEq)]
pub struct UserRef {
    #[serde(rename = "$ref")]
    pub ref_field: String,
    #[serde(rename = "$id")]
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, Dummy, Clone, PartialEq)]
pub struct Media {
    #[serde(rename = "type")]
    pub media_type: String,
    pub link: String,
}

#[derive(Serialize, Deserialize, Debug, Dummy, Clone, PartialEq)]
pub struct CollectionRef {
    #[serde(rename = "$ref")]
    pub ref_field: String,
    #[serde(rename = "$id")]
    pub id: i32,
    pub oid: i32,
}

#[derive(Serialize, Deserialize, Debug, Dummy, Clone, PartialEq)]
pub struct CreatorRef {
    #[serde(rename = "_id")]
    pub id: i32,
    pub avatar: String,
    pub name: String,
    pub email: String,
}
