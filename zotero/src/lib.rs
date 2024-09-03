mod bibtex_loader;

use abi_stable::std_types::*;
use bibtex_loader::{BibTexLoader, ZoteroItem};
use findex_plugin::{define_plugin, ApplicationCommand, FResult};
use shellexpand::tilde;
use std::process::Command;

fn init(_: &RHashMap<RString, RString>) -> RResult<(), RString> {
    ROk(())
}

fn search(mut items: Vec<ZoteroItem>, search: &str) -> Vec<ZoteroItem> {
    items.sort_by(|a, b| b.date_modified.cmp(&a.date_modified));

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
        items.into_iter().take(10).collect()
    };
}

fn handle_query(query: RStr) -> RVec<FResult> {
    if let Ok(data) = BibTexLoader::load_zotero(&*tilde("~/.cache/zotero/My-Library.json")) {
        return search(BibTexLoader::load_items(data), &query)
            .into_iter()
            .map(|i| FResult {
                cmd: ApplicationCommand::Command(RString::from(format!("xdg-open {}", i.select))),
                icon: RString::from(&*tilde(&format!(
                    "~/.cache/illef-findex-plugin/zotero-icons/{}.svg",
                    i.icon
                ))),
                score: isize::MAX,
                name: RString::from(i.title),
                desc: RSome(RString::from(format!(
                    "{}",
                    i.tags
                        .iter()
                        .map(|t| format!("#{}", t))
                        .collect::<Vec<_>>()
                        .join(" "),
                ))),
            })
            .collect();
    }

    RVec::new()
}

define_plugin!("zotero!", init, handle_query);
