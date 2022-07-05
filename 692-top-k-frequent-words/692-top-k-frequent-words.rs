use std::{
    collections::{HashMap, BinaryHeap},
    cmp::{Reverse, Ordering}
};

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut map = HashMap::new();
        
        for word in words{
            *map.entry(word).or_insert(0) +=1;
        }
        
        let mut heap = BinaryHeap::new();
        
        for (word, cnt) in map{
            heap.push((-cnt, word));
            
            if heap.len() > k as usize{
                heap.pop();
            }
        }
        
        heap
        .into_sorted_vec()
        .into_iter()
        .map(|(_, word)| word)
        .collect()
    }
}