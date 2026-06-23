use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;
use std::time::{ SystemTime, UNIX_EPOCH };
use rand::{RngExt};

use crate::db::Db;

pub struct ExpireStore<K> {
    db: HashMap<K, u64>,
    keys: Vec<K>,
    key_to_index: HashMap<K, usize>
}

impl<K> ExpireStore<K> 
where K: std::hash::Hash + Eq + Clone
{
    pub fn new() -> Self {
        Self {
            db: HashMap::new(),
            keys: vec![],
            key_to_index: HashMap::new()
        }
    }
    pub fn set_expire(&mut self, key: K, time: u64) -> Option<u64> {
        if !self.db.contains_key(&key) {
            self.keys.push(key.clone());
            self.key_to_index.insert(key.clone(), self.keys.len());
        }
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
    pub fn remove_key(&mut self, key: &K) -> bool {
        let idx = match self.key_to_index.get(key) {
            Some(idx) => *idx,
            None => return false,
        };

        let moved_key = self.keys.last().unwrap().clone();

        self.keys.swap_remove(idx);

        if &moved_key != key {
            self.key_to_index.insert(moved_key, idx);
        }

        self.key_to_index.remove(key);
        self.db.remove(key);

        true
    }

    pub fn remove_using_clt<V>(&mut self, map: &mut Db<K, V>)
    where
        K: Clone + Hash + Eq,
    {
        let mut rng = rand::rng();

        loop {
            if self.keys.is_empty() {
                return;
            }

            let mut expired_count = 0;

            let sample_size = self.keys.len().min(20);

            for _ in 0..sample_size {
                if self.keys.is_empty() {
                    break;
                }

                let index = rng.random_range(0..self.keys.len());
                let key = self.keys[index].clone();

                if let Some(&expires_at) = self.db.get(&key) {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    if now > expires_at {
                        map.delete(&key);
                        self.remove_key(&key);
                        expired_count += 1;
                    }
                }
            }

            if expired_count * 4 < sample_size {
                break;
            }
        }
    }
}

// expire store contains the keys that has the expiration set
// take 20 random keys from the expire store 
// remove the expired keys
// if the key is more than 25 percent. then take another 20. and continue the loop until the invalid key percentage is less than 25 percent