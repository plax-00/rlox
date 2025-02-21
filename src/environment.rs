use anyhow::{bail, Result};
use rustc_hash::FxHashMap;

use crate::value::Value;

#[derive(Debug, Default)]
struct Scope {
    values: FxHashMap<String, Value>,
}

#[derive(Debug)]
pub struct Environment {
    scopes: Vec<Scope>,
}

impl Default for Environment {
    fn default() -> Self {
        let scopes = vec![Scope::default()];
        Self { scopes }
    }
}

impl Environment {
    pub fn define(&mut self, name: &str, value: Value) {
        self.scopes
            .last_mut()
            .unwrap()
            .values
            .insert(name.into(), value);
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<Value> {
        let frames = self.scopes.iter_mut().rev();
        for frame in frames {
            if !frame.values.contains_key(name) {
                continue;
            }

            return frame.values.insert(name.into(), value).map(Ok).unwrap();
        }
        bail!("{} is undefined", name);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        let frames = self.scopes.iter().rev();
        for frame in frames {
            if !frame.values.contains_key(name) {
                continue;
            }
            return frame.values.get(name);
        }
        None
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() == 1 {
            panic!("Attempted to pop global scope");
        }
        self.scopes.pop();
    }
}
