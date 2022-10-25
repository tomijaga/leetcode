#[derive(Default)]
struct MinStack {
    stack: Vec<(i32, i32)>
}

impl MinStack {

    fn new() -> Self {
        Default::default()
    }
    
    fn push(&mut self, val: i32) {
        let len = self.stack.len();
        
        if len == 0 {
            self.stack.push((val, val));
        }else{
            let prev_min = self.stack[len - 1].1;
            self.stack.push((val, val.min(prev_min)));
        }
    }
    
    fn pop(&mut self) {
        self.stack.pop();
    }
    
    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }
    
    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}