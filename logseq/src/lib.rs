mod cache;
mod logseq;

use abi_stable::std_types::*;
use cache::{FilePageCache, LogseqTag};
use findex_plugin::{define_plugin, ApplicationCommand, FResult};
use rand::rng;
use rand::seq::SliceRandom;
use std::{thread, time::Duration};

fn init(_: &RHashMap<RString, RString>) -> RResult<(), RString> {
    thread::spawn(move || loop {
        if let Ok(pages) = logseq::get_logseq_pages() {
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
        Ok(mut pages) => {
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
            remaining_pages.shuffle(&mut rng());

            // Combine recent pages (first 5) with shuffled remaining pages
            let mut sorted_pages = recent_pages.to_vec();
            sorted_pages.extend_from_slice(remaining_pages);

            let filtered_pages: Vec<_> = sorted_pages
                .into_iter()
                .filter(|page| {
                    if search_term.is_empty() {
                        true
                    } else {
                        page.title.to_lowercase().contains(&search_term)
                            || page.tags.iter().any(|tag| {
                                tag.name.to_string().to_lowercase().contains(&search_term)
                            })
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
                            RSome(RString::from(
                                page.tags
                                    .iter()
                                    .filter(|tag| &tag.name != &"Page")
                                    .map(|tag| format!("#{}", tag.name))
                                    .collect::<Vec<String>>()
                                    .join(" "),
                            ))
                        };

                        let icon = get_icon_for_tags(&page.tags);

                        FResult {
                            cmd: ApplicationCommand::Command(RString::from(format!(
                                "bash -c 'xdg-open logseq://graph/illef2?block-id={}'",
                                page.uuid
                            ))),
                            icon: RString::from(icon),
                            score: isize::MAX,
                            name: RString::from(page.title),
                            desc,
                        }
                    })
                    .collect::<Vec<_>>(),
            )
        }
        Err(_) => RVec::new(),
    }
}

fn get_icon_for_tags(tags: &[LogseqTag]) -> String {
    if let Some(tag) = tags
        .iter()
        .filter(|t| {
            let excluded = [
                "Task",
                "Page",
                "DONE",
                "TODAY",
                "INBOX",
                "SOMEDAY",
                "DELEGATE",
                "GTD-PROJECT",
            ];
            !excluded.contains(&t.name.as_str())
        })
        .next()
    {
        if let Some(icon) = &tag.icon {
            if icon.icon_type == "emoji" {
                if let Some(emoji) = emojis::get_by_shortcode(&icon.id) {
                    return format!("emoji:{}", emoji.as_str());
                }
            }
        }
    }

    if tags
        .iter()
        .any(|tag| matches!(tag.name.as_str(), "Task" | "GTD" | "GTD-PROJECT"))
    {
        return "gnome-todo".to_string();
    }
    "emoji:📝".to_string()
}

define_plugin!("logseq!", init, handle_query);
