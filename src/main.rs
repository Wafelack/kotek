mod parser;
mod eval;
mod builtins;
use parser::Parser;
use eval::Evaluator;
use std::io;

fn main() -> Result<()> {
    let mut stack = Vec::with_capacity(256);
    let mut vars = vec![];
    let mut evaluator = Evaluator::new(vec![], vars.clone(), stack.clone());
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let expressions = Parser::new(buffer.trim()).parse()?;
        evaluator = Evaluator::new(expressions, vars, stack);
        let (new_stack, new_vars) = evaluator.eval()?;
        println!("=> {}", new_stack.last().and_then(|v| Some(v.clone().get_lit(true))).unwrap_or("".to_string()));
        buffer.clear();
        stack = new_stack;
        vars = new_vars;
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
