use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Default)]
pub struct Db<K, V> {
	store: HashMap<K, V>,
}

pub struct Item<V> {
	pub value: V,
	pub exp: i32
}

impl<K, Item> Db<K, Item>
where
	K: Eq + Hash,
{
	pub fn new() -> Self {
		Self {
			store: HashMap::new(),
		}
	}

	pub fn set(&mut self, key: K, value: Item) -> Option<Item> {
		self.store.insert(key, value)
	}

	pub fn get(&self, key: &K) -> Option<&Item> {
		self.store.get(key)
	}

	pub fn delete(&mut self, key: &K) -> bool {
		if self.store.contains_key(key){
			self.store.remove(key);
			return true;
		}
		false
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
		assert_eq!(db.delete(&"name"), true);
		assert_eq!(db.get(&"name"), None);
	}
}
