use std::collections::HashMap;

struct TimeMap {
    map: HashMap<String, Vec<(String, i32)>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {

    fn new() -> Self {
        Self{
            map: HashMap::new()
        }
    }
    
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        
        let entry = (value, timestamp);
        
        if let Some(values) = self.map.get_mut(&key){
            values.push(entry);
        }else{
            self.map.insert(key, vec![entry]);
        }
    }
    
    fn get(&self, key: String, timestamp: i32) -> String {
        if !self.map.contains_key(&key){
            return "".to_owned();
        }
        
        let mut values = self.map.get(&key).unwrap();
        
        if timestamp < values[0].1{
            return String::new();
        }else if timestamp == values[0].1{
            return values[0].0.clone();
        } else if values.len() ==1{
            return values[0].0.clone();
        }
        
        let mut l = 0;
        let mut r = values.len() - 1;
        
        while l < r{
            let mid = l + (r - l + 1)/2;
            let time = values[mid].1;
            
            if  time <= timestamp{
                l = mid;
            }else {
                r = mid - 1;
            }
        }
        
        let res = values[l].0.clone();
        
        // println!("get {} \n{:?}, \nres: {:?} ",timestamp, values, res);
        res
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */