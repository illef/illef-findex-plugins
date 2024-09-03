mod raindrop;

use abi_stable::std_types::*;
use cache::{FileItemCache, FileItemScoreCache};
use client::Client;
use findex_plugin::{define_plugin, ApplicationCommand, FResult};
use raindrop::*;
use shellexpand::tilde;
use std::{process::Command, thread, time::Duration};
use types::Item;

fn search(items: Vec<Item>, search: &str) -> Vec<Item> {
    let mut items = items;
    items.sort_by(|a, b| b.last_update.cmp(&a.last_update));

    return if search.trim().len() == 0 {
        items
    } else {
        for keyword in search.split(" ") {
            items = if keyword.starts_with("#") && keyword.len() > 1 {
                items
                    .into_iter()
                    .filter(|i| {
                        i.tags
                            .iter()
                            .filter(|t| t.to_lowercase().contains(&keyword[1..]))
                            .count()
                            > 0
                    })
                    .collect()
            } else {
                items
                    .into_iter()
                    .filter(|i| i.title.to_lowercase().contains(&keyword))
                    .collect()
            };
        }
        items
    };
}

fn init(config: &RHashMap<RString, RString>) -> RResult<(), RString> {
    if let Some(api_token) = config.get("api-token") {
        let api_token = api_token.to_string();
        thread::spawn(move || {
            let client = Client::new(&api_token);
            loop {
                if let Ok(items) = client.get_all_items() {
                    let cache = FileItemCache::default();
                    cache.update_cache(items).expect("update_cache");
                }
                FileItemScoreCache::default().update_item_scores();
                Command::new(&*tilde(
                    "~/.cache/illef-findex-plugin/scripts/download_favicons.sh",
                ))
                .output()
                .expect("Failed to execute command");

                thread::sleep(Duration::from_secs(60));
            }
        });

        ROk(())
    } else {
        RErr(RString::from("Cannot find api-token config"))
    }
}

fn handle_query(query: RStr) -> RVec<FResult> {
    let cache = FileItemCache::default();
    let score_cache = FileItemScoreCache::default();

    let mut items = search(cache.load_cache().unwrap(), query.as_str());

    let item_scores = score_cache.load_item_scores();
    let mut score_sorted_items = vec![];

    for item_score in item_scores.iter().take(7) {
        if let Some(item) = items.iter().find(|s| s.id == item_score.id) {
            score_sorted_items.push(item.clone());
        }
    }
    items = items
        .into_iter()
        .filter(|i| score_sorted_items.iter().find(|s| s.id == i.id).is_none())
        .collect();
    score_sorted_items.extend(items);
    score_sorted_items = score_sorted_items.into_iter().take(15).collect();

    RVec::from(
        score_sorted_items
            .into_iter()
            .map(|i| FResult {
                cmd: ApplicationCommand::Command(RString::from(&*tilde(&format!(
                    "bash -c \"~/.cache/illef-findex-plugin/scripts/open_browser.sh {} {}\"",
                    i.link, i.id
                )))),
                icon: RString::from(&*tilde(&format!(
                    "~/.cache/illef-findex-plugin/favicons/{}.ico",
                    i.id
                ))),
                score: isize::MAX,
                name: RString::from(i.title),
                desc: RSome(RString::from(format!(
                    "{} {}",
                    i.tags
                        .iter()
                        .map(|t| format!("#{}", t))
                        .collect::<Vec<_>>()
                        .join(" "),
                    i.excerpt
                ))),
            })
            .collect::<Vec<_>>(),
    )
}

define_plugin!("raindrop!", init, handle_query);
