use crate::evaluator::Object;
use std::collections::HashMap;

pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Object {
        // TODO Handle getting non existant key properly...
        let val = self.store.get(key).unwrap_or(&Object::Null);
        val.clone()
    }

    pub fn set(&mut self, key: String, value: Object) {
        self.store.insert(key, value);
    }
}
