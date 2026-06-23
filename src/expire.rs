use std::collections::HashMap;
use std::ops::Add;
use std::time::{ UNIX_EPOCH, SystemTime };

pub struct ExpireStore<K> {
    db: HashMap<K, u64>
}

impl<K> ExpireStore<K> 
where K: std::hash::Hash + Eq
{
    pub fn new() -> Self {
        Self {
            db: HashMap::new()
        }
    }
    pub fn set_expire(&mut self, key: K, time: u64) -> Option<u64> {
        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap().as_secs().add(time);
        self.db.insert(key, expires_at)
    }
    pub fn contains_key(&self, key: &K) -> bool{
        self.db.contains_key(key)
    }
    pub fn get_key_time(&self, key: &K) -> Option<&u64>{
        self.db.get(key)
    }
}

// expire store contains the keys that has the expiration set
// take 20 random keys from the expire store 
// remove the expired keys
// if the key is more than 25 percent. then take another 20. and continue the loop until the key is expired