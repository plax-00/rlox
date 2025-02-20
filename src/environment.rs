use anyhow::{bail, Result};
use rustc_hash::FxHashMap;

use crate::value::Value;

#[derive(Debug, Default)]
pub struct Environment(FxHashMap<String, Value>);

impl Environment {
    pub fn define(&mut self, name: &str, value: Value) {
        self.0.insert(name.into(), value);
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<Value> {
        match self.0.insert(name.into(), value.clone()) {
            Some(_) => Ok(value),
            None => bail!("{} is not defined", name),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.0.get(name)
    }
}
