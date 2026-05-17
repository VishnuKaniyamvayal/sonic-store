use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Default)]
pub struct Db<K, V> {
	store: HashMap<K, V>,
}

impl<K, V> Db<K, V>
where
	K: Eq + Hash,
{
	pub fn new() -> Self {
		Self {
			store: HashMap::new(),
		}
	}

	pub fn set(&mut self, key: K, value: V) -> Option<V> {
		self.store.insert(key, value)
	}

	pub fn get(&self, key: &K) -> Option<&V> {
		self.store.get(key)
	}

	pub fn delete(&mut self, key: &K) -> Option<V> {
		self.store.remove(key)
	}
}

#[cfg(test)]
mod tests {
	use super::Db;

	#[test]
	fn set_get_and_delete_work() {
		let mut db = Db::new();

		assert_eq!(db.set("name", "sonic"), None);
		assert_eq!(db.get(&"name"), Some(&"sonic"));
		assert_eq!(db.delete(&"name"), Some("sonic"));
		assert_eq!(db.get(&"name"), None);
	}
}
