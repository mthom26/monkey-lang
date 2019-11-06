use std::collections::HashMap;

pub struct Symbol {
    index: u16,
    scope: String,
}

impl Symbol {
    pub fn new(index: u16, scope: String) -> Self {
        Symbol { index, scope }
    }
}

pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
    next_index: u16,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            next_index: 0,
        }
    }

    pub fn define(&mut self, name: String, scope: String) -> u16 {
        let symbol = Symbol::new(self.next_index, scope);
        self.symbols.insert(name, symbol);
        self.next_index += 1;
        self.next_index - 1
    }

    pub fn resolve(&self, name: String) -> Option<u16> {
        let symbol = self.symbols.get(&name);

        match symbol {
            Some(val) => Some(val.index),
            None => None,
        }
    }
}
