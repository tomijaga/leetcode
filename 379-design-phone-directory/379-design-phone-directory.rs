use std::collections::{BTreeSet, BinaryHeap};

struct PhoneDirectory {
    bitvec: Vec<i32>,
    nbits: i32,
    n: i32,
    released: BTreeSet<i32>
}

impl PhoneDirectory {

    fn new(maxNumbers: i32) -> Self {
        Self{
            bitvec: vec![0; (maxNumbers/32) as usize + 1],
            nbits: maxNumbers,
            n: 0,
            released: BTreeSet::new()
        }
    }
    
    fn pos(&self, n: i32) -> (usize, usize){
        ((n / 32) as usize, (n % 32) as usize)
    }
    
    fn set(&mut self, n: i32, b: bool){
        let (r, c) = self.pos(n);
        
        if b {
            self.bitvec[r] |= 1 << c;
        }else{
            self.bitvec[r] &= !(1 << c);
        }
    }
    
    fn get(&mut self) -> i32 {
        
        if self.n < self.nbits{
            let val = self.n;
            self.n+=1;

            self.set(val, true);

            return val;
        }
        
        if let Some(&n) = self.released.iter().next(){
            self.set(n, true);
            self.released.remove(&n);
            return n;
        }
        
        return -1;
    }
    
    fn check(&self, number: i32) -> bool {
        let (r, c) = self.pos(number);
        // println!("{:b}", &self.bitvec[0]);
        (self.bitvec[r] & 1<<c) == 0
    }
    
    fn release(&mut self, number: i32) {
        if number < self.n{
            self.set(number, false);
            self.released.insert(number);
        }
    }
}