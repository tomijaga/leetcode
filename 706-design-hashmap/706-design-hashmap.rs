const MAX_SIZE: usize = 10_usize.pow(6) + 1;

struct MyHashMap {
    map: [i32; MAX_SIZE]
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    fn new() -> Self {
        Self{
            map: [-1; MAX_SIZE]
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        self.map[key as usize] = value;
    }
    
    fn get(&self, key: i32) -> i32 {
        self.map[key as usize]
    }
    
    fn remove(&mut self, key: i32) {
        self.put(key, -1);
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */