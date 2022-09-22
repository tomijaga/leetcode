use std::collections::{BTreeSet, BinaryHeap};

struct PhoneDirectory {
    nbits: i32,
    n: i32,
    released: BTreeSet<i32>
}

impl PhoneDirectory {

    fn new(maxNumbers: i32) -> Self {
        Self{
            nbits: maxNumbers,
            n: 0,
            released: BTreeSet::new()
        }
    }

    fn get(&mut self) -> i32 {
        
        if self.n < self.nbits{
            let val = self.n;
            self.n+=1;

            return val;
        }
        
        if let Some(&n) = self.released.iter().next(){
            self.released.remove(&n);
            return n;
        }
        
        return -1;
    }
    
    fn check(&self, number: i32) -> bool {
        // println!("{:?}", (number >= self.n, !self.released.contains(&number)));
        number >= self.n || self.released.contains(&number)
    }
    
    fn release(&mut self, number: i32) {
        if number < self.n{
            self.released.insert(number);
        }
    }
}