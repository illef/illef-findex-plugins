mod cache;
mod logseq;

use abi_stable::std_types::*;
use cache::FilePageCache;
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
        thread::sleep(Duration::from_secs(60 * 5));
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
                            || page
                                .tags
                                .iter()
                                .any(|tag| tag.to_string().to_lowercase().contains(&search_term))
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
                                    .filter(|tag| tag != &"Page")
                                    .map(|tag| format!("#{}", tag))
                                    .collect::<Vec<String>>()
                                    .join(" "),
                            ))
                        };

                        let icon = get_icon_for_tags(&page.tags);

                        FResult {
                            cmd: ApplicationCommand::Command(RString::from(format!(
                                "bash -c 'xdg-open logseq://graph/illef2?page={}'",
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

fn get_icon_for_tags(tags: &[String]) -> &'static str {
    // Prioritize tags by specificity, returning the first matching icon
    for tag in tags {
        match tag.as_str() {
            "글감" => return "typewriter",
            "book" | "tech-book" | "bookReview" => return "fbreader",
            "term" => return "dictionary",
            "Code" | "tech" => return "format-text-code",
            "Math" => return "math0",
            "movie" => return "video-x-generic-symbolic",
            "video" => return "video-x-generic-symbolic",
            "lecture" => return "audio-x-generic-symbolic",
            "project" | "project-item" => return "folder-yellow",
            "person" => return "im-user",
            "news" => return "news-subscribe",
            "app" => return "application-x-generic",
            "note" | "notes" => return "keep",
            "comment" => return "text-x-generic",
            "chat" => return "text-x-generic",
            "english" => return "text-x-generic",
            "post" => return "artistictext-tool",
            "Query" => return "system-search",
            "Task" | "GTD" | "GTD-PROJECT" => return "gnome-todo",
            "Card" | "Cards" => return "text-x-generic",
            "Quote" => return "text-x-generic",
            "Journal" => return "text-x-generic",
            "Template" => return "folder-templates",
            "Asset" => return "package-x-generic",
            "Whiteboard" => return "image-x-generic",
            "magazineArticle" => return "news-subscribe",
            _ => continue,
        }
    }

    // Default fallback icon
    "logseq"
}

define_plugin!("logseq!", init, handle_query);
