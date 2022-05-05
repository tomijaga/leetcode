use std::collections::VecDeque;

#[derive(Default)]
struct MyStack {
    inbox: VecDeque<i32>,
    outbox: VecDeque<i32>,
    top: i32,
    swaps: i32
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    fn new() -> Self {
        Default::default()
    }
    
    fn inbox_mut(&mut self)-> &mut VecDeque<i32>{
        if (self.swaps % 2 == 1){
            &mut self.outbox
        }else{
            &mut self.inbox
        }
    }
    
    fn outbox_mut(&mut self)-> &mut VecDeque<i32>{
        if (self.swaps % 2 == 1){
            &mut self.inbox
        }else{
            &mut self.outbox
        }
    }
    
    
    fn push(&mut self, x: i32) {
        self.top = x;
        
        self.inbox_mut().push_back(x);
    }
    
    fn pop(&mut self) -> i32 {
        while self.inbox_mut().len() > 1{
            if let Some(elem) = self.inbox_mut().pop_front(){
                self.top = elem;
                self.outbox_mut().push_back(elem);
            }
        }
        
        let item = self.inbox_mut().pop_front().unwrap();
        
        self.swaps+=1;
        
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