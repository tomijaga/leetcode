use std::cmp::min;

struct CustomStack {
    stack : Vec<i32>,
    i: usize
}


impl CustomStack {

    fn new(maxSize: i32) -> Self {
        Self{
            stack: vec![-1; maxSize as usize],
            i: 0
        }
    }
    
    fn push(&mut self, x: i32) {
        if self.i < self.stack.len(){
            self.stack[self.i] = x;
            self.i+=1;
        }
    }
    
    fn pop(&mut self) -> i32 {
        if self.i > 0 {
            self.i-=1;
            self.stack[self.i]
        }else{
            -1
        }
    }
    
    fn increment(&mut self, k: i32, val: i32) {
        let k = k as usize;
        let start = if self.i > k  {self.i - k} else {0};
        
        for i in 0..min(k, self.i){
            self.stack[i]+=val;
        }
    }
}

/**
 * Your CustomStack object will be instantiated and called as such:
 * let obj = CustomStack::new(maxSize);
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * obj.increment(k, val);
 */