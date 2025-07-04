use abi_stable::std_types::*;
use findex_plugin::{define_plugin, ApplicationCommand, FResult};

const SYMBOLS_TXT: &str = include_str!("../assets/symbols.txt");

pub struct Symbol {
    str: String,
    description: String,
}

impl Symbol {
    fn new(str: &str, description: &str) -> Self {
        Symbol {
            str: str.to_string(),
            description: description.to_string(),
        }
    }
    fn favorite_symbols() -> Vec<Symbol> {
        vec![
            Symbol::new("·", "middle dot"),
            Symbol::new("—", "em dash"),
            Symbol::new("⋯", "middle dots"),
            Symbol::new("⋯⋯", "double middle dots"),
            Symbol::new("「」", "corner brackets"),
            Symbol::new("『』", "white corner brackets"),
            Symbol::new("《》", "double angle brackets"),
            Symbol::new("〈〉", "angle brackets"),
            Symbol::new("〔〕", "tortoise shell brackets"),
            Symbol::new("〘〙", "white curly brackets"),
            Symbol::new("〚〛", "white square brackets"),
            Symbol::new("✾", "floral heart"),
            Symbol::new("✿", "flower"),
            Symbol::new("❀", "white flower"),
            Symbol::new("❁", "outline flower"),
            Symbol::new("❃", "teardrop-spoked asterisk"),
            Symbol::new("∽", "similar to"),
            Symbol::new("∾", "inverted lazy S"),
            Symbol::new("⁓", "swung dash"),
            Symbol::new("✔", "check mark"),
            Symbol::new("✕", "multiplication x"),
            Symbol::new("Ꮺ", "cherokee letter wv"),
            Symbol::new("ᔕ", "canadian syllabics sha"),
            Symbol::new("∮", "contour integral"),
            Symbol::new("ᝢ", "hanunoo letter la"),
            Symbol::new("ᓬ", "canadian syllabics l"),
            Symbol::new("ៜ", "khmer symbol lek too"),
            Symbol::new("ᨐ", "tai tham letter high ha"),
            Symbol::new("ᨏ", "tai tham letter high da"),
            Symbol::new("᯼", "buginese vowel sign e"),
            Symbol::new("᯽", "buginese vowel sign o"),
            Symbol::new("᯾", "buginese vowel sign ae"),
            Symbol::new("❖", "black diamond"),
            Symbol::new("⸎", "left low paraphrase bracket"),
            Symbol::new("⸾", "dashed overline"),
        ]
    }
}

impl Into<FResult> for Symbol {
    fn into(self) -> FResult {
        FResult {
            cmd: ApplicationCommand::Command(RString::from(format!(
                "bash -c 'printf {} | xclip -selection clipboard'",
                self.str
            ))),
            icon: RString::from("artistictext-tool"),
            score: isize::MAX,
            name: RString::from(self.str.to_string()),
            desc: RSome(RString::from(self.description)),
        }
    }
}

impl From<&str> for Symbol {
    fn from(line: &str) -> Self {
        let mut chars = line.chars();
        let char = chars.next().unwrap();
        let description = chars.take(60).collect::<String>();
        Symbol {
            str: char.to_string(),
            description,
        }
    }
}

fn init(_: &RHashMap<RString, RString>) -> RResult<(), RString> {
    ROk(())
}

fn handle_query(query: RStr) -> RVec<FResult> {
    let query_str = query.to_string();
    if query.trim().len() == 0 {
        RVec::from_iter(Symbol::favorite_symbols().into_iter().map(|s| s.into()))
    } else {
        let lines: Vec<&str> = SYMBOLS_TXT.lines().collect();
        let mut matched = lines
            .into_iter()
            .filter(|line| line.contains(&query_str))
            .map(|line| Symbol::from(line))
            .collect::<Vec<_>>();
        matched.extend(
            Symbol::favorite_symbols()
                .into_iter()
                .filter(|s| s.description.contains(&query_str)),
        );
        RVec::from_iter(matched.into_iter().map(|s| s.into()))
    }
}

define_plugin!("unicode_picker!", init, handle_query);
