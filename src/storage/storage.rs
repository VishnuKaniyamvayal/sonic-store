use std::collections::HashMap;

pub struct Storage {
    data: HashMap<String, String>,
}

impl Storage {
    pub fn new() -> Self{
        Storage { data: HashMap::new() }
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) -> Option<String>{
        self.data.insert(key.into(), value.into())
    }

    pub fn get(&self, key: &str) -> Option <&str>{
        self.data.get(key).map(|s| s.as_str())
    }
}