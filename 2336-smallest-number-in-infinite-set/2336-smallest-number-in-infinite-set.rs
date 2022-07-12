use std::collections::{HashSet, BinaryHeap};

#[derive(Default)]
struct SmallestInfiniteSet {
    set: HashSet<i32>,
    heap: BinaryHeap<i32>,
    n: i32
}

impl SmallestInfiniteSet {

    fn new() -> Self {
        Self{
            n: 1,
            ..Default::default()
        }
    }
    
    fn pop_smallest(&mut self) -> i32 {
        if self.heap.len() == 0{
            let tmp = self.n;
            self.n+=1;
            tmp
        }else{
            let p = self.heap.pop().unwrap() * -1;
            
            self.set.remove(&p);
            p
        }
    }
    
    fn add_back(&mut self, num: i32) {
        if num < self.n && !self.set.contains(&num){
            self.set.insert(num);
            self.heap.push(-num);
        }
    }
}
