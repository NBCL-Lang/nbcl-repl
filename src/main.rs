use nbcl::{NbclEngine, context::EvalContext};

fn main() {
    let engine = NbclEngine::new();
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    let mut buffer = String::new();
    let mut accumulated_ctx: EvalContext = EvalContext::from(&engine);

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
                            Ok(_) => {}
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
