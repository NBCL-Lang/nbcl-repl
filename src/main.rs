mod register;

use nbcl::{NbclEngine, context::EvalContext, ast::resolved::ResolvedTree};
use register::register_all_into;
use std::sync::{Arc, Mutex};

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

    let mut rl = rustyline::DefaultEditor::new().unwrap();
    let mut buffer = String::new();
    let mut accumulated_ctx: EvalContext = EvalContext::from(&engine);

    // print help
    println!("Help: Exit using Ctrl+C, Ctrl+D, or exit()");
    println!("Help: Run help() for information");

    loop {
        let prompt = if buffer.is_empty() { ">> " } else { ".. " };
        match rl.readline(prompt) {
            Ok(line) => {
                rl.add_history_entry(&line).unwrap();
                buffer.push_str(&line);
                buffer.push('\n');

                if is_incomplete(&buffer) {
                    continue;
                }

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
            Err(_) => break,
        }
    }
}

fn is_incomplete(input: &str) -> bool {
    let mut depth_curly = 0i32;
    let mut depth_square = 0i32;
    let mut depth_paren = 0i32;

    for ch in input.chars() {
        match ch {
            '{' => depth_curly += 1,
            '}' => depth_curly -= 1,
            '[' => depth_square += 1,
            ']' => depth_square -= 1,
            '(' => depth_paren += 1,
            ')' => depth_paren -= 1,
            _ => {}
        }
    }

    depth_curly > 0 || depth_square > 0 || depth_paren > 0
}
