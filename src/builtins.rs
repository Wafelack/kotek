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
}
