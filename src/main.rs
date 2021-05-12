mod parser;
use parser::{Parser, Expr};
use std::io;

fn main() -> Result<()> {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let expressions = Parser::new(buffer.trim()).parse()?;
        println!("{:?}", expressions);
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
