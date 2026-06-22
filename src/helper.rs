use std::borrow::Cow::{self, Owned};

use rustyline::completion::FilenameCompleter;
use rustyline::highlight::{CmdKind, Highlighter};
use rustyline::hint::HistoryHinter;
use rustyline::validate::MatchingBracketValidator;
use rustyline::{Completer, Helper, Hinter, Validator};

#[derive(Helper, Completer, Hinter, Validator)]
pub struct CustomHelper {
    #[rustyline(Completer)]
    completer: FilenameCompleter,
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
}

impl Highlighter for CustomHelper {
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[90m".to_owned() + hint + "\x1b[0m")
    }

    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        let keys: Vec<&str> = vec![
            "const", "let", "as", "any", "set", "fn", "for", 
            "in", "while", "if", "else", "match", "return", 
            "import"
        ];

        let data_types: Vec<&str> = vec![
            "true", "false", "null"
        ];

        let mut current_line = line.to_string();

        // strings
        let regex = regex::Regex::new(r#""[^"]*""#).unwrap();
        current_line = regex.replace_all(&current_line, |cap: &regex::Captures| {
            format!("\x1b[33m{}\x1b[0m", &cap[0])
        }).to_string();

        // function calls
        let regex = regex::Regex::new(r"\b(\w+)\s*\(").unwrap();
        current_line = regex.replace_all(&current_line, |cap: &regex::Captures| {
            format!("\x1b[34m{}\x1b[0m(", &cap[1])
        }).to_string();

        for key in keys {
            let pattern = format!("\\b{}\\b", key);
            let regex = regex::Regex::new(&pattern).unwrap();
            current_line = regex.replace_all(&current_line, &format!("\x1b[36m{}\x1b[0m", key)).to_string();
        }

        for data_type in data_types {
            let pattern = format!("\\b{}\\b", data_type);
            let regex = regex::Regex::new(&pattern).unwrap();
            current_line = regex.replace_all(&current_line, &format!("\x1b[33m{}\x1b[0m", data_type)).to_string();
        }

        current_line.into()
    }

    fn highlight_char(&self, _line: &str, _pos: usize, kind: CmdKind) -> bool {
        match kind {
            CmdKind::Other => true,
            CmdKind::ForcedRefresh => true,
            CmdKind::MoveCursor => false,
        }
    }
}

impl CustomHelper {
    pub fn new() -> Self {
        Self {
            completer: FilenameCompleter::new(),
            hinter: HistoryHinter::new(),
            validator: MatchingBracketValidator::new(),
        }
    }
}
