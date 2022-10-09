use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Default)]
struct MedianFinder {
    left: BinaryHeap<i32>,
    right: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {

    fn new() -> Self {
        Default::default()
    }
    
    fn add_num(&mut self, num: i32) {
        let mut left = &mut self.left;
        let mut right = &mut self.right;
        
        left.push(num);
        right.push(Reverse(left.pop().unwrap()));
        
        if left.len() < right.len(){
            let Reverse(n) = right.pop().unwrap();
            left.push(n);
        }
    }
    
    fn find_median(&self) -> f64 {
        let left = &self.left;
        let right = &self.right;
        
        if left.len() == right.len(){
            let a = *left.peek().unwrap();
            let Reverse(b) = *right.peek().unwrap();
            (a + b) as f64 / 2_f64
        }else{
            *left.peek().unwrap() as f64
        }
    }
}