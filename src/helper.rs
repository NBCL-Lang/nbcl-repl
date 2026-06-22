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
        let data_types: Vec<&str> = vec!["true", "false", "null"];

        let kw_pattern = keys.join("|");
        let dt_pattern = data_types.join("|");
        let pattern = format!(
            r#"("(?:[^"]*)")|(\b(?:{kw_pattern})\b)|(\b(?:{dt_pattern})\b)|(\b(\w+)\s*\()"#
        );
        let regex = regex::Regex::new(&pattern).unwrap();

        let mut output = String::new();
        let mut last_end = 0;

        for cap in regex.captures_iter(line) {
            let m = cap.get(0).unwrap();
            output.push_str(&line[last_end..m.start()]);

            if cap.get(1).is_some() {
                output.push_str(&format!("\x1b[32m{}\x1b[0m", m.as_str()));
            } else if cap.get(2).is_some() {
                output.push_str(&format!("\x1b[36m{}\x1b[0m", m.as_str()));
            } else if cap.get(3).is_some() {
                output.push_str(&format!("\x1b[33m{}\x1b[0m", m.as_str()));
            } else if cap.get(5).is_some() {
                output.push_str(&format!("\x1b[34m{}\x1b[0m(", &cap[5]));
            }

            last_end = m.end();
        }

        output.push_str(&line[last_end..]);
        output.into()
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
