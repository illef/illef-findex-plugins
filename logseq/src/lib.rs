mod cache;
mod logseq;

use abi_stable::std_types::*;
use cache::FilePageCache;
use findex_plugin::{define_plugin, ApplicationCommand, FResult};
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
                            icon: RString::from("logseq"),
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

define_plugin!("logseq!", init, handle_query);
