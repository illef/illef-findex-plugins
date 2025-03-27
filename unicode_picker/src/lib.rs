use abi_stable::std_types::*;
use findex_plugin::{define_plugin, ApplicationCommand, FResult};

const SYMBOLS_TXT: &str = include_str!("../assets/symbols.txt");
const FAVORITE_SYMBOLS_TXT: &str = &"·⋯—「」『 』《 》〈 〉〔 〕〘 〙〚〛✾✿❀❁❃∽∾⁓✔️";

pub struct Symbol {
    char: char,
    description: String,
}

impl Into<FResult> for Symbol {
    fn into(self) -> FResult {
        FResult {
            cmd: ApplicationCommand::Command(RString::from(format!(
                "bash -c 'printf {} | xclip -selection clipboard'",
                self.char
            ))),
            icon: RString::from("artistictext-tool"),
            score: isize::MAX,
            name: RString::from(self.char.to_string()),
            desc: RSome(RString::from(self.description)),
        }
    }
}

impl From<&str> for Symbol {
    fn from(line: &str) -> Self {
        let mut chars = line.chars();
        let char = chars.next().unwrap();
        let description = chars.take(60).collect::<String>();
        Symbol { char, description }
    }
}

fn init(_: &RHashMap<RString, RString>) -> RResult<(), RString> {
    ROk(())
}

fn handle_query(query: RStr) -> RVec<FResult> {
    let query_str = query.to_string();
    let lines: Vec<&str> = SYMBOLS_TXT.lines().collect();
    if query.trim().len() == 0 {
        let matched = lines
            .into_iter()
            .filter(|line| FAVORITE_SYMBOLS_TXT.contains(line.chars().next().unwrap()))
            .map(|line| Symbol::from(line))
            .collect::<Vec<_>>();
        RVec::from_iter(matched.into_iter().map(|s| s.into()))
    } else {
        let matched = lines
            .into_iter()
            .filter(|line| line.contains(&query_str))
            .map(|line| Symbol::from(line))
            .collect::<Vec<_>>();
        RVec::from_iter(matched.into_iter().map(|s| s.into()))
    }
}

define_plugin!("unicode_picker!", init, handle_query);
