use abi_stable::std_types::*;
use findex_plugin::{define_plugin, ApplicationCommand, FResult};
use shellexpand::tilde;
use std::process::Command;

fn init(_: &RHashMap<RString, RString>) -> RResult<(), RString> {
    ROk(())
}

fn translate(str: &str) -> String {
    let output = String::from_utf8(
        Command::new("/usr/bin/trans")
            .args(["en:ko", "-b", "--no-ansi", str])
            .output()
            .expect("Failed to execute command")
            .stdout,
    )
    .expect("decode utf-8");

    output
        .chars()
        .collect::<Vec<char>>()
        .chunks(70)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn dict(str: &str) -> String {
    String::from_utf8(
        Command::new("/usr/bin/trans")
            .args(["en:ko", "-no-ansi", str])
            .output()
            .expect("Failed to execute command")
            .stdout,
    )
    .expect("decode utf-8")
    .replace("\t", "    ")
}

fn handle_query(query: RStr) -> RVec<FResult> {
    if query.trim().len() == 0 {
        return RVec::new();
    }
    let output = if query.find(" ").is_some() {
        translate(&query)
    } else {
        dict(&query)
    };

    RVec::from(vec![FResult {
        cmd: ApplicationCommand::Command(RString::from(&*tilde(&format!(
            "fish -c 'speak \"{}\"'",
            &query
        )))),
        icon: RString::from(&*tilde(
            "~/.cache/illef-findex-plugin/favicons/translate.png",
        )),
        score: isize::MAX,
        name: RString::from(output),
        desc: RNone,
    }])
}

define_plugin!("translator!", init, handle_query);
