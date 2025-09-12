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

    let mut pages: Vec<LogseqPage> = blocks
        .into_iter()
        .map(|block| {
            let title = block.title.unwrap_or_else(|| "Untitled".to_string());
            let uuid = block.uuid.unwrap_or_default();
            let tags = block
                .tags
                .unwrap_or_default()
                .into_iter()
                .filter_map(|tag_ref| tag_ref.id.map(|id| format!("tag-{}", id)))
                .collect();

            LogseqPage {
                title,
                uuid,
                tags,
                updated_at: block.updated_at,
            }
        })
        .collect();

    // Sort pages by updated_at in descending order (most recent first)
    pages.sort_by(|a, b| match (a.updated_at, b.updated_at) {
        (Some(a_time), Some(b_time)) => b_time.cmp(&a_time),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    });

    // Take top 5 most recently updated pages and shuffle the rest
    let pages_len = pages.len();
    let split_index = if pages_len > 5 { 5 } else { pages_len };

    let (recent_pages, remaining_pages) = pages.split_at_mut(split_index);

    // Shuffle the remaining pages
    use rand::rng;
    use rand::seq::SliceRandom;
    remaining_pages.shuffle(&mut rng());

    // Combine recent pages (first 5) with shuffled remaining pages
    let mut result = recent_pages.to_vec();
    result.extend_from_slice(remaining_pages);

    Ok(result)
}
