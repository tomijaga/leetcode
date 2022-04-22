struct MinStack {
    currentMin: i32,
    stack: Vec<(i32, i32)> // (val, min)
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

use std::i32;

impl MinStack {

    fn new() -> Self {
        MinStack{
            currentMin: i32::MAX,
            stack: vec![]
        }
    }
    
    fn push(&mut self, val: i32) {
        if (val < self.currentMin){
            self.currentMin = val;
        }
        
        self.stack.push((val, self.currentMin));
    }
    
    fn pop(&mut self) {
        self.stack.pop();
        
        let len = self.stack.len();
        if (len > 0){
            let (_, min)  = self.stack[len -1];
        
            self.currentMin = min;
        }else{
            self.currentMin = i32::MAX;
        }
        
    }
    
    fn top(&self) -> i32 {
        let last_index = self.stack.len()-1;
        let (val, _) = self.stack[last_index];
        val
        
    }
    
    fn get_min(&self) -> i32 {
        self.currentMin
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */