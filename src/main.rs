mod parser;
mod eval;
mod builtins;
use parser::Parser;
use eval::Evaluator;
use rustyline::{error::ReadlineError, Editor};

fn print_err(e: Error) {
    eprintln!("\x1b[0;31m{}:{} | {}\x1b[0m", e.0, e.1, e.2);
}

fn repl() {
    let mut evaluator = Evaluator::new(vec![]);
    let mut symbols = vec![];
    let mut reader = Editor::<()>::new();
    loop {
        let line = reader.readline("kitty> ");
        match line {
            Ok(line) => {
                reader.add_history_entry(line.as_str());
                if line == "quit" {
                    return;
                }

                let mut parser = Parser::new(line.as_str(), symbols.clone());
                let (expressions, new_syms)= match parser.parse() {
                    Ok(res) => res,
                    Err(e) => {
                        print_err(e);
                        continue;
                    }
                };
                symbols = new_syms;
                evaluator.update(expressions);
                match evaluator.eval() {
                    Ok(val) => match val {
                        Some(top) => println!("=> {} :: {}", top.clone().get_lit(true), top.get_type()),
                        None => {}
                    }
                    Err(e) => print_err(e),
                } 
            }
            Err(ReadlineError::Interrupted) => {
                println!("=> #Interrupt");
            }
            Err(ReadlineError::Eof) => return,
            Err(_) => {
                eprintln!("An error occured while reading input, please retry.");
            }
        }
    }
}

fn main() -> Result<()> {
    repl();
    Ok(())
}

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug)]
pub struct Error(usize, usize, String);

#[macro_export]
macro_rules! error {
    ($line:expr, $column:expr, $($arg:tt)*) => {
        Err(Error($line, $column, format_args!($($arg)*).to_string()))
    }
}
