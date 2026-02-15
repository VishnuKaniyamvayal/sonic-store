mod storage;

use storage::Storage;

fn main(){
    let mut storage = Storage::new();

    storage.set("User", "Vishnu");

    let result = storage.get(&"User").unwrap();

    println!("{}", result);

}