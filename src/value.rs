use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use anyhow::{bail, Result};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        !matches!(self, Value::Bool(false) | Value::Nil)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Value::Nil => "nil",
            Value::String(s) => s,
            Value::Number(n) => &format!("{}", n),
            Value::Bool(b) => &format!("{}", b),
        };
        write!(f, "{}", repr)
    }
}

impl Sub for Value {
    type Output = Result<Self>;
    fn sub(self, rhs: Self) -> Self::Output {
        let result = match (self, rhs) {
            (Self::Number(l), Self::Number(r)) => Self::Number(l - r),
            (l, r) => bail!("Cannot subtract {:?} from {:?}", r, l),
        };
        Ok(result)
    }
}

impl Add for Value {
    type Output = Result<Self>;
    fn add(self, rhs: Self) -> Self::Output {
        let result = match (self, rhs) {
            (Self::Number(l), Self::Number(r)) => Self::Number(l + r),
            (Self::String(l), Self::String(r)) => Self::String(l + &r),
            (l, r) => bail!("Cannot add {:?} and {:?}", l, r),
        };
        Ok(result)
    }
}

impl Mul for Value {
    type Output = Result<Self>;
    fn mul(self, rhs: Self) -> Self::Output {
        let result = match (self, rhs) {
            (Self::Number(l), Self::Number(r)) => Self::Number(l * r),
            (Self::String(s), Self::Number(n)) => Self::String(s.repeat(n as usize)),
            (l, r) => bail!("Cannot multiply {:?} and {:?}", l, r),
        };
        Ok(result)
    }
}

impl Div for Value {
    type Output = Result<Self>;
    fn div(self, rhs: Self) -> Self::Output {
        let result = match (self, rhs) {
            (Self::Number(l), Self::Number(r)) => Self::Number(l / r),
            (l, r) => bail!("Cannot multiply {:?} and {:?}", l, r),
        };
        Ok(result)
    }
}
