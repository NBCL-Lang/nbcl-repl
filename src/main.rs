mod register;
mod helper;

use nbcl::{NbclEngine, context::EvalContext, ast::resolved::ResolvedTree};
use register::register_all_into;
use std::sync::{Arc, Mutex};
use rustyline::{CompletionType, EditMode, error::ReadlineError};

#[derive(Clone, Debug, Default)]
pub struct Data {
    prev_tree: Option<ResolvedTree>,
}

impl Data {
    pub fn new() -> Arc<Mutex<Self>> {
         Arc::new(Mutex::new(Self::default()))
    }
}

fn main() {
    let mut engine = NbclEngine::new();
    let data = Data::new();
    
    register_all_into(&mut engine, data.clone());

    let config = rustyline::Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build();

    let mut rl = rustyline::Editor::with_config(config).expect("Rustyline to load");
    let mut buffer = String::new();
    let mut accumulated_ctx: EvalContext = EvalContext::from(&engine);
    let mut last_was_interrupt = false;

    let event = rustyline::KeyEvent::new('\t', rustyline::Modifiers::NONE);
    let action = rustyline::Cmd::Insert(1, "\t".to_string());

    rl.set_helper(Some(helper::CustomHelper::new()));
    rl.bind_sequence(event, rustyline::EventHandler::Simple(action));

    // print help
    println!("Help: Exit using Ctrl+C, Ctrl+D, or exit()");
    println!("Help: Run help() for information");

    loop {
        let prompt = if buffer.is_empty() { ">> " } else { ".. " };
        match rl.readline(prompt) {
            Ok(line) => {
                last_was_interrupt = false;
                rl.add_history_entry(&line).unwrap();
                buffer.push_str(&line);
                buffer.push('\n');

                match engine.parse_str(&buffer) {
                    Ok(ast) => {
                        match engine.eval_ast_with_eval_ctx(ast, &mut accumulated_ctx) {
                            Ok(tree) => {
                                if let Ok(ref mut mutex) = data.try_lock() {
                                    mutex.prev_tree = Some(tree);
                                }
                            }
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Parse Error: {}", e),
                }

                buffer.clear();
            }
            Err(ReadlineError::Interrupted) => {
                if !buffer.is_empty() {
                    buffer.clear();
                    last_was_interrupt = false;
                    println!("(input cleared)");
                } else if last_was_interrupt {
                    break;
                } else {
                    last_was_interrupt = true;
                    println!("(press Ctrl+C again to exit)");
                }
            }
            Err(_) => break,
        }
    }
}
