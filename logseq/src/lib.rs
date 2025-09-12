mod cache;

use abi_stable::std_types::*;
use cache::{FilePageCache, LogseqPage};
use findex_plugin::{define_plugin, ApplicationCommand, FResult};
use serde::{Deserialize, Serialize};
use std::{process::Command, thread, time::Duration};

#[derive(Debug, Deserialize, Serialize)]
struct LogseqBlock {
    #[serde(rename = "block/title")]
    title: Option<String>,
    #[serde(rename = "block/uuid")]
    uuid: Option<String>,
    #[serde(rename = "block/tags")]
    tags: Option<Vec<TagRef>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TagRef {
    #[serde(rename = "db/id")]
    id: Option<i64>,
}

fn get_logseq_pages() -> Result<Vec<LogseqPage>, String> {
    let output = Command::new("bash")
        .arg("-c")
        .arg("npx @logseq/cli query illef2 '[:find (pull ?b [:block/tags :block/uuid :block/title]) :where [?tag :block/name \"page\"] [?b :block/tags ?tag]]' | jet --to json")
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

    // Parse JSON string to our structs
    let blocks: Vec<LogseqBlock> =
        serde_json::from_str(&json_str).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let pages = blocks
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

            LogseqPage { title, uuid, tags }
        })
        .collect();

    Ok(pages)
}

fn init(_: &RHashMap<RString, RString>) -> RResult<(), RString> {
    thread::spawn(move || loop {
        if let Ok(pages) = get_logseq_pages() {
            let cache = FilePageCache::default();
            if let Err(e) = cache.update_cache(pages) {
                eprintln!("Failed to update logseq cache: {}", e);
            }
        }
        thread::sleep(Duration::from_secs(60));
    });

    ROk(())
}

fn handle_query(query: RStr) -> RVec<FResult> {
    let search_term = query.as_str().to_lowercase();
    let cache = FilePageCache::default();

    match cache.load_cache() {
        Ok(pages) => {
            let filtered_pages: Vec<_> = pages
                .into_iter()
                .filter(|page| {
                    if search_term.is_empty() {
                        true
                    } else {
                        page.title.to_lowercase().contains(&search_term)
                            || page
                                .tags
                                .iter()
                                .any(|tag| tag.to_lowercase().contains(&search_term))
                    }
                })
                .take(15)
                .collect();

            RVec::from(
                filtered_pages
                    .into_iter()
                    .map(|page| {
                        let desc = if page.tags.is_empty() {
                            RNone
                        } else {
                            RSome(RString::from(format!("Tags: {}", page.tags.join(", "))))
                        };

                        FResult {
                            cmd: ApplicationCommand::Command(RString::from(format!(
                                "bash -c 'xdg-open logseq://graph/illef2?page={}'",
                                page.uuid
                            ))),
                            icon: RString::from("terminal"),
                            score: isize::MAX,
                            name: RString::from(page.title),
                            desc,
                        }
                    })
                    .collect::<Vec<_>>(),
            )
        }
        Err(_) => {
            // Fallback to direct query if cache fails
            match get_logseq_pages() {
                Ok(pages) => {
                    let filtered_pages: Vec<_> = pages
                        .into_iter()
                        .filter(|page| {
                            if search_term.is_empty() {
                                true
                            } else {
                                page.title.to_lowercase().contains(&search_term)
                                    || page
                                        .tags
                                        .iter()
                                        .any(|tag| tag.to_lowercase().contains(&search_term))
                            }
                        })
                        .take(15)
                        .collect();

                    RVec::from(
                        filtered_pages
                            .into_iter()
                            .map(|page| {
                                let desc = if page.tags.is_empty() {
                                    RNone
                                } else {
                                    RSome(RString::from(format!("Tags: {}", page.tags.join(", "))))
                                };

                                FResult {
                                    cmd: ApplicationCommand::Command(RString::from(format!(
                                        "open 'logseq://graph/illef2?page={}'",
                                        page.uuid
                                    ))),
                                    icon: RString::from("📄"),
                                    score: isize::MAX,
                                    name: RString::from(page.title),
                                    desc,
                                }
                            })
                            .collect::<Vec<_>>(),
                    )
                }
                Err(e) => RVec::from(vec![FResult {
                    cmd: ApplicationCommand::Command(RString::from("echo 'Error occurred'")),
                    icon: RString::from("❌"),
                    score: isize::MAX,
                    name: RString::from(format!("Error: {}", e)),
                    desc: RNone,
                }]),
            }
        }
    }
}

define_plugin!("logseq!", init, handle_query);
