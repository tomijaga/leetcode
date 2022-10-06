use std::collections::{HashMap, BTreeSet};

#[derive(Default)]
struct TimeMap {
    map : HashMap<String, BTreeSet<(i32, String)>>
}

impl TimeMap {

    fn new() -> Self {
        Default::default()
    }
    
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key)
            .or_insert(BTreeSet::new())
            .insert((timestamp, value));
    }
    
    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(set) = self.map.get(&key){
            for (prev_time, value) in set.iter().rev(){
                if *prev_time <= timestamp{
                    return value.clone();
                }
            }    
        }
        
        String::new()
    }
}