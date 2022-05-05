use std::collections::VecDeque;
use std::mem;

#[derive(Default)]
struct MyStack {
    inbox: VecDeque<i32>,
    outbox: VecDeque<i32>,
    top: i32
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        Default::default()
    }
    
    fn push(&mut self, x: i32) {
        self.top = x;
        self.inbox.push_back(x);
    }
    
    fn pop(&mut self) -> i32 {
        while self.inbox.len() > 1{
            self.top = self.inbox.pop_front().unwrap();
            self.outbox.push_back(self.top);
        }
        
        let item = self.inbox.pop_front().unwrap();
        std::mem::swap(&mut self.inbox, &mut self.outbox);
        
        item
    }
    
    fn top(&self) -> i32 {
        self.top
    }
    
    fn empty(&self) -> bool {
        self.inbox.is_empty() && self.outbox.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */