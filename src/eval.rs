use crate::{parser::{Expr, ExprT}, Result, Error, error};

#[derive(Clone)]
pub enum Value {
    Integer(i32),
    Real(f32),
    String(String),
    Quote(Vec<Expr>),
    Symbol(String),
}
impl Value {
    pub fn get_type(self) -> String {
        match self {
            Value::Integer(_) => "Integer",
            Value::Real(_) => "Real",
            Value::String(_) => "String",
            Value::Quote(_) => "Quote",
            Value::Symbol(_) => "Symbol"
        }.to_string()
    }
    pub fn get_lit(self, quotes: bool) -> String {
        match self {
            Value::Integer(z) => format!("{}", z),
            Value::Real(r) => format!("{}", r),
            Value::String(s) => format!("{}{}{}", if quotes { "\"" } else { "" }, s, if quotes { "\"" } else { "" }),
            Value::Symbol(sym) => sym,
            Value::Quote(content) => format!("[{}]", content.into_iter().map(|e| e.r#type.get_lit()).collect::<Vec<String>>().join(" ")),
        }
    }
}
pub struct Evaluator {
    stack: Vec<Value>,
    vars: Vec<Vec<Expr>>,
    input: Vec<Expr>,
    builtins: Vec<fn(&mut Evaluator, usize, usize) -> Result<()>>
}
impl Evaluator {
    pub fn new(input: Vec<Expr>, vars: Vec<Vec<Expr>>, stack: Vec<Value>) -> Self {
        Self {
            input,
            vars,
            stack,
            builtins: vec![Self::add],
        }
    }
    pub fn push(&mut self, val: Value) -> Result<()> {
        self.stack.push(val);
        Ok(())
    }
    pub fn pop(&mut self, line: usize, column: usize) -> Result<Value> {
        match self.stack.pop() {
            Some(v) => Ok(v),
            None => error!(line, column, "Stack_underflow")
        }
    }
    fn eval_expr(&mut self, expr: Expr) -> Result<()> {
        match expr.r#type {
            ExprT::Integer(i) => self.stack.push(Value::Integer(i)),
            ExprT::Real(r) => self.stack.push(Value::Real(r)),
            ExprT::String(s) => self.stack.push(Value::String(s)),
            ExprT::Quote(content) => self.stack.push(Value::Quote(content)),
            ExprT::Symbol(sym) => self.stack.push(Value::Symbol(sym)),
            ExprT::Builtin(idx) => self.builtins[idx as usize](self, expr.line, expr.column)?,
            ExprT::Store(idx, content) => {
                let idx = idx as usize;
                if idx < self.vars.len() {
                    self.vars[idx] = content;
                } else {
                    self.vars.push(content);
                }
            }
            ExprT::Var(idx) => {
                self.vars.clone().into_iter().nth(idx as usize).unwrap().iter().map(|expr| {
                    self.eval_expr(expr.clone())
                }).collect::<Result<()>>()?;
            }
        }
        Ok(())
    }
    pub fn eval(&mut self) -> Result<(Vec<Value>, Vec<Vec<Expr>>)> {
        self.input.clone().into_iter().map(|expr| self.eval_expr(expr)).collect::<Result<()>>()?;
        Ok((self.stack.clone(), self.vars.clone()))
    }
}
