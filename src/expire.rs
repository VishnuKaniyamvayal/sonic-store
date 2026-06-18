use std::collections::HashMap;

pub struct Expire_Store<K, V> {
    db: HashMap<K, V>
}

impl<K, V> Expire_Store<K, V> 
where K: std::hash::Hash + Eq
{
    pub fn init() -> Self {
        Self {
            db: HashMap::new()
        }
    }
    pub fn set_expire(&mut self, key: K, time: V) -> Option<V> {
        self.db.insert(key, time)
    }
}