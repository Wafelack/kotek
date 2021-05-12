mod parser;
mod eval;
mod builtins;
use parser::Parser;
use eval::Evaluator;
use std::io;

fn main() -> Result<()> {
    let mut evaluator = Evaluator::new(vec![]);
    let mut parser = Parser::new("");
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        parser.update_code(buffer.trim());
        let exprs = parser.parse()?;
        evaluator.update(exprs);
        let top = evaluator.eval()?;
        match top {
            Some(v) => println!("=> {}", v.get_lit(true)),
            None => {},
        }
        buffer.clear();
    }
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
