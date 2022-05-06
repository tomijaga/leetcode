use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    big: BinaryHeap<Reverse<i32>>,
    small: BinaryHeap<i32>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        Self{
            big: BinaryHeap::new(),
            small: BinaryHeap::new()
        }
    }
    
    fn len(&self)-> usize{
        self.small.len() +  self.big.len()
    }
    
    fn add_num(&mut self, num: i32) {
        self.small.push(num);
        
        let n = self.small.pop().unwrap();
        self.big.push(Reverse(n));
        
        let s_len = self.small.len();
        let mid = self.len()/2;
        // println!("l:{}, s:{}, m:{}", self.len(), s_len, mid);
        
        if s_len <= mid{
            if let Some(Reverse(n)) = self.big.pop(){
                self.small.push(n);
            }
        }
    }
    
    fn find_median(&mut self) -> f64 {
        let len = self.len();
        let mid = len/2;
        
        let mut elems = vec![];
        
        // println!("s: {:?}\nb:{:?}\n", &self.small, &self.big);
        
        if len % 2 == 0{
            elems.push(self.small.pop().unwrap());
            elems.push(self.small.pop().unwrap());
        }else{
            elems.push(self.small.pop().unwrap());
        }
        
        for &n in elems.iter(){
            self.small.push(n);
        }
        
        elems.iter().sum::<i32>() as f64 / elems.len() as f64
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */