use crate::cache::LogseqPage;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Deserialize, Serialize)]
pub struct LogseqBlock {
    #[serde(rename = "block/title")]
    pub title: Option<String>,
    #[serde(rename = "block/uuid")]
    pub uuid: Option<String>,
    #[serde(rename = "block/tags")]
    pub tags: Option<Vec<TagRef>>,
    #[serde(rename = "block/updated-at")]
    pub updated_at: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TagRef {
    #[serde(rename = "db/id")]
    pub id: Option<i64>,
}

pub fn get_logseq_pages() -> Result<Vec<LogseqPage>, String> {
    let output = Command::new("bash")
        .arg("-c")
        .arg("npx @logseq/cli query illef2 '[:find (pull ?b [:block/tags :block/uuid :block/title :block/updated-at]) :where [?tag :block/name \"page\"] [?b :block/tags ?tag]]' | jet --to json")
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let json_str =
        String::from_utf8(output.stdout).map_err(|e| format!("Invalid UTF-8 in output: {}", e))?;

    let blocks: Vec<LogseqBlock> =
        serde_json::from_str(&json_str).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let pages: Vec<LogseqPage> = blocks
        .into_iter()
        .map(|block| {
            let title = block.title.unwrap_or_else(|| "Untitled".to_string());
            let uuid = block.uuid.unwrap_or_default();
            let tags = block
                .tags
                .unwrap_or_default()
                .into_iter()
                .filter_map(|tag_ref| tag_ref.id)
                .collect();

            LogseqPage {
                title,
                uuid,
                tags,
                updated_at: block.updated_at,
            }
        })
        .collect();

    Ok(pages)
}
