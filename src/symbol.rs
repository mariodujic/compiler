use std::fmt;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    table: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable { table: vec![] }
    }

    pub fn add(&mut self, symbol: Symbol) {
        self.table.push(symbol)
    }

    pub fn get(&mut self, name: &str) -> Option<&Symbol> {
        self.table.iter().find(|s| *s.name == *name)
    }

    pub fn replace_with_same_name(&mut self, new_symbol: Symbol) -> Option<Symbol> {
        for symbol in self.table.iter_mut() {
            if symbol.name == new_symbol.name {
                let old_symbol = std::mem::replace(symbol, new_symbol);
                return Some(old_symbol);
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: Box<str>,
    pub value: Value,
    pub mutable: bool,
}

impl Symbol {
    pub fn new(name: Box<str>, value: Value, mutable: bool) -> Symbol {
        Symbol { name, value, mutable }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    String(Box<str>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(_) => write!(f, "Integer"),
            Value::String(_) => write!(f, "String"),
        }
    }
}