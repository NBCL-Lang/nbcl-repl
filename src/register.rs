use nbcl::{NbclEngine, Value, Type};
use std::sync::{Arc, Mutex};
use crate::Data;

pub fn register_all_into(engine: &mut NbclEngine, data: Arc<Mutex<Data>>) {
    engine.register_native_fn("exit", vec![], Type::Null, |_| {
        std::process::exit(0);
    });

    engine.register_native_fn("tree", vec![], Type::Null, move |_| {
        if let Ok(ref mutex) = data.try_lock() {
            nbcl::print(format!("{:#?}", mutex.prev_tree));
        }

        Ok(Value::Null)
    });

    engine.register_native_fn("clear", vec![], Type::Null, |_| {
        let _ = std::process::Command::new("clear").status();
        Ok(Value::Null)
    });

    engine.register_native_fn("help", vec![], Type::Null, |_| {
        let help = vec![
            "tree()   Return the tree returned by last line.",
            "clear()  Clear the terminal buffer.",
            "exit()   Exit the REPL.",
            "help()   Print this help message.",
            "",
            "Press Ctrl+C, Ctrl+D, or run exit() to exit the REPL."
        ];

        for itm in help {
            nbcl::print(itm);
        }

        Ok(Value::Null)
    });
}
