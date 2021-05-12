use crate::{eval::{Evaluator, Value}, Result, error, Error};

impl Evaluator {
    pub fn add(&mut self, line: usize, column: usize) -> Result<()> {
        let lhs = self.pop(line, column)?;
        let rhs = self.pop(line, column)?;

        match lhs {
            Value::Integer(lhs) => match rhs {
                Value::Integer(rhs) => self.push(Value::Integer(lhs + rhs)),
                _ => error!(line, column, "Expected an Integer, found a {}.", rhs.get_type()),
            }
            Value::Real(lhs) => match rhs {
                Value::Real(rhs) => self.push(Value::Real(lhs + rhs)),
                _ => error!(line, column, "Expected a Real, found a {}.", rhs.get_type()),
            }
            _ => error!(line, column, "Expected a Real or an Integer, found a {}.", lhs.get_type()),
        }
    }
    pub fn sub(&mut self, line: usize, column: usize) -> Result<()> {
        let lhs = self.pop(line, column)?;
        let rhs = self.pop(line, column)?;

        match lhs {
            Value::Integer(lhs) => match rhs {
                Value::Integer(rhs) => self.push(Value::Integer(lhs - rhs)),
                _ => error!(line, column, "Expected an Integer, found a {}.", rhs.get_type()),
            }
            Value::Real(lhs) => match rhs {
                Value::Real(rhs) => self.push(Value::Real(lhs - rhs)),
                _ => error!(line, column, "Expected a Real, found a {}.", rhs.get_type()),
            }
            _ => error!(line, column, "Expected a Real or an Integer, found a {}.", lhs.get_type()),
        }
    }
    pub fn mul(&mut self, line: usize, column: usize) -> Result<()> {
        let lhs = self.pop(line, column)?;
        let rhs = self.pop(line, column)?;

        match lhs {
            Value::Integer(lhs) => match rhs {
                Value::Integer(rhs) => self.push(Value::Integer(lhs * rhs)),
                _ => error!(line, column, "Expected an Integer, found a {}.", rhs.get_type()),
            }
            Value::Real(lhs) => match rhs {
                Value::Real(rhs) => self.push(Value::Real(lhs * rhs)),
                _ => error!(line, column, "Expected a Real, found a {}.", rhs.get_type()),
            }
            _ => error!(line, column, "Expected a Real or an Integer, found a {}.", lhs.get_type()),
        }
    }
    pub fn div(&mut self, line: usize, column: usize) -> Result<()> {
        let lhs = self.pop(line, column)?;
        let rhs = self.pop(line, column)?;

        match lhs {
            Value::Integer(lhs) => match rhs {
                Value::Integer(rhs) => self.push(Value::Integer(lhs / rhs)),
                _ => error!(line, column, "Expected an Integer, found a {}.", rhs.get_type()),
            }
            Value::Real(lhs) => match rhs {
                Value::Real(rhs) => self.push(Value::Real(lhs / rhs)),
                _ => error!(line, column, "Expected a Real, found a {}.", rhs.get_type()),
            }
            _ => error!(line, column, "Expected a Real or an Integer, found a {}.", lhs.get_type()),
        }
    }
    pub fn app(&mut self, line: usize, column: usize) -> Result<()> {
        let val = self.pop(line, column)?;

        match val {
            Value::Quote(exprs) => exprs.into_iter().map(|expr| self.eval_expr(expr)).collect::<Result<()>>(),
            _ => error!(line, column, "Expected a Quote, found a {}.", val.get_type()),
        }
    }
    pub fn dup(&mut self, line: usize, column: usize) -> Result<()> {
        let to_dup = self.pop(line, column)?;
        self.push(to_dup.clone())?;
        self.push(to_dup)
    }
}
