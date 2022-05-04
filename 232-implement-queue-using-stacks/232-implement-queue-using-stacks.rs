struct MyQueue {
    inbox: Vec<i32>,
    outbox: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    
    fn new() -> Self {
        Self{
            inbox: vec![],
            outbox: vec![],
        }
    }
    
    fn _flip(&mut self){
        if self.outbox.is_empty(){
            while let Some(elem) = self.inbox.pop(){
                self.outbox.push(elem);
            }
        }
    }
    
    fn push(&mut self, x: i32) {
        self._flip();
        self.inbox.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        self._flip();
        self.outbox.pop().unwrap()
    }
    
    fn peek(&mut self) -> i32 {
        self._flip();
        if let Some(&last) = self.outbox.last(){
            return last;
        }
        
        return -1;
    }
    
    fn empty(&self) -> bool {
        self.inbox.is_empty() && self.outbox.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */