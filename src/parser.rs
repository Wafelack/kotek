use crate::{Result, error, Error};

#[derive(Clone, Debug)]
pub enum ExprT {
    Symbol(String),
    String(String),
    Integer(i32),
    Real(f32),
    Quote(Vec<Expr>),
    Store(u16, Vec<Expr>),
    Var(u16),
    Builtin(u16)
}
impl ExprT {
    pub fn get_lit(self) -> String {
        match self {
            Self::Symbol(sym) => sym,
            Self::String(s) => format!("\"{}\"", s),
            Self::Integer(i) => format!("{}", i),
            Self::Real(r) => format!("{}", r),
            Self::Quote(content) => format!("[{}]", content.into_iter().map(|e| e.r#type.get_lit()).collect::<Vec<String>>().join(" ")),
            Self::Var(idx) => format!("${}", idx),
            Self::Store(idx, content) => format!("${} => ({})", idx, content.into_iter().map(|e| e.r#type.get_lit()).collect::<Vec<String>>().join(" ")),
            Self::Builtin(idx) => format!("builtin#{}", idx),
        }
    }
}
#[derive(Clone, Debug)]
pub struct Expr {
    pub r#type: ExprT,
    pub line: usize,
    pub column: usize,
}
impl Expr {
    pub fn new(r#type: ExprT, line: usize, column: usize) -> Self {
        Self {
            r#type,
            line,
            column
        }
    }
}
pub struct Parser {
    input: String,
    symbols: Vec<String>,
    builtins: Vec<String>,
    output: Vec<Expr>,
    line: usize,
    column: usize,
    start: usize,
    current: usize,
}

const FINISHING: [Option<char>; 7] = [Some('('), Some(']'), Some(')'), Some(' '), Some('\t'), Some('\r'), Some('\n')];

impl Parser {
    pub fn new(input: impl ToString, symbols: Vec<String>) -> Self {
        let mut to_ret = Self {
            input: input.to_string(),
            output: vec![],
            symbols,
            builtins: vec![],
            line: 0,
            column: 0,
            start: 0,
            current: 0,
        };
        to_ret.register_builtin("+");
        to_ret.register_builtin("-");
        to_ret.register_builtin("*");
        to_ret.register_builtin("/");
        to_ret.register_builtin("%");
        to_ret.register_builtin("dup");
        to_ret.register_builtin("app");
        to_ret.register_builtin("cat");
        to_ret.register_builtin("pop");
        to_ret.register_builtin("swap");
        to_ret.register_builtin("print_stack");
        to_ret.register_builtin("eq");
        to_ret.register_builtin("not");
        to_ret.register_builtin("gt");
        to_ret.register_builtin("lt");
        to_ret
    }
    fn register_builtin(&mut self, builtin: impl ToString) {
        self.builtins.push(builtin.to_string());
    }
    fn spaces(&mut self) -> Result<()> {
        while !self.is_at_end() && self.peek(0) == Some(' ') {
            self.pop()?;
        }
        Ok(())
    }
    fn advance(&mut self, expected: char) -> Result<()> {
        let popped = self.pop()?;
        if popped == expected {
            Ok(())
        } else {
            error!(self.line, self.column, "Expected '{}', found '{}'", expected, popped)
        }
    }
    fn pop(&mut self) -> Result<char> {
        match self.input.chars().nth(self.current) {
            Some(c) => {
                self.current += 1;
                if c == '\n' {
                    self.line += 1;
                    self.column = 0;
                } else {
                    self.column += 1;
                }
                Ok(c)
            }
            None => error!(self.line, self.column, "Unexpected EOF while parsing.")
        }
    }
    fn peek(&self, ahead: usize) -> Option<char> {
        self.input.chars().nth(self.current + ahead)
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.input.chars().count()
    }
    fn parse_one(&mut self) -> Result<Option<Expr>> {
        let c = self.pop()?;

        match c {
            ' ' | '\t' | '\n' | '\r' => Ok(None),
            '#' => {
                let (line, column) = (self.line, self.column);
                while !self.is_at_end() && !FINISHING.contains(&self.peek(0)){
                    self.pop()?;
                }
                let start = self.start;
                self.start = self.current;
                Ok(Some(Expr::new(ExprT::Symbol(self.input[start + 1..self.current].to_string()), line, column)))
            }
            ';' => {
                while !self.is_at_end() && self.peek(0) != Some('\n') {
                    self.pop()?;
                }
                Ok(None)
            }
            '[' => {
                let (line, column) = (self.line, self.column);
                let mut content = vec![];
                while self.peek(0) != Some(']') {
                    self.start = self.current;
                    match self.parse_one()?  {
                        Some(expr) => content.push(expr),
                        None => {},
                    }
                }
                self.advance(']')?;
                Ok(Some(Expr::new(ExprT::Quote(content), line, column)))
            }
            '"' => {
                let (line, column) = (self.line, self.column);
                while self.peek(0) != Some('"') {
                    self.pop()?;
                }
                self.advance('"')?;
                Ok(Some(Expr::new(ExprT::String(self.input[self.start + 1..self.current - 1].to_string()), line, column)))
            }
            x => if x.is_digit(10) {
                self.number()
            } else {
                self.identifier()
            }
        }
    }
    fn identifier(&mut self) -> Result<Option<Expr>> {
        let (line, column) = (self.line, self.column);
        while !self.is_at_end() && !FINISHING.contains(&self.peek(0)) {
            self.pop()?;
        }
        let raw = self.input[self.start..self.current].to_string();
        if raw == "let" {
            Ok(Some(self.declare()?))
        } else if self.builtins.contains(&raw) {
            Ok(Some(Expr::new(ExprT::Builtin(self.builtins.iter().position(|builtin| builtin.to_string() == raw).unwrap() as u16), line, column)))
        } else {
            if self.symbols.contains(&raw) {
                Ok(Some(Expr::new(ExprT::Var(self.symbols.iter().position(|sym| sym.to_string() == raw).unwrap() as u16), line, column)))
            } else {
                error!(line, column, "Use of an undefined variable: {}.", raw)
            }
        }
    }
    fn declare(&mut self) -> Result<Expr> {
        let (line, column) = (self.line, self.column);
        self.spaces()?;
        self.start = self.current;
        while !FINISHING.contains(&self.peek(0)) {
            self.pop()?;
        }
        let name = self.input[self.start..self.current].to_string();
        self.spaces()?;
        self.advance('(')?;
        let mut content = vec![];
        while self.peek(0) != Some(')') {
            self.start = self.current;
            match self.parse_one()? {
                Some(expr) => content.push(expr),
                None => {}
            }
        }
        self.advance(')')?;
        let idx = if self.symbols.contains(&name) {
            self.symbols.iter().position(|sym| sym.to_string() == name).unwrap()
        } else {
            self.symbols.push(name);
            self.symbols.len() - 1
        };
        Ok(Expr::new(ExprT::Store(idx as u16, content), line, column))
    }
    fn number(&mut self) -> Result<Option<Expr>> {
        let (line, column) = (self.line, self.column);
        while let Some(c) = self.peek(0) {
            if c.is_digit(10) {
                self.pop()?;
            } else {
                break;
            }
        }
        if self.peek(0) == Some('.') {
            self.pop()?;
        }
        while let Some(c) = self.peek(0) {
            if c.is_digit(10) {
                self.pop()?;
            } else {
                break;
            }
        }
        let raw = self.input[self.start..self.current].to_string();
        Ok(Some(Expr::new(match raw.parse::<i32>() {
            Ok(z) => ExprT::Integer(z),
            Err(_) => ExprT::Real(raw.parse::<f32>().unwrap_or(3.1415926535897932))
        }, line, column)))    
    }
    pub fn parse(&mut self) -> Result<(Vec<Expr>, Vec<String>)> {
        while !self.is_at_end() {
            match self.parse_one()? {
                Some(expr) => self.output.push(expr),
                None => {},
            }
            self.start = self.current;
        }
        Ok((self.output.clone(), self.symbols.clone()))
    }
}
