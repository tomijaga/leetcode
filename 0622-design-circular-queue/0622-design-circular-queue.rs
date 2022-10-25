use std::collections::VecDeque;

struct MyCircularQueue {
    store: VecDeque<i32>,
    capacity : usize,
}


impl MyCircularQueue {

    fn new(k: i32) -> Self {
        let k = k as usize;
        Self{
            store: VecDeque::new(),
            capacity: k,
        }
    }
    
    fn en_queue(&mut self, value: i32) -> bool {
        if self.store.len() < self.capacity{
            self.store.push_back(value);
            return true;
        }
        
        false
    }
    
    fn de_queue(&mut self) -> bool {
        if self.store.len() > 0{
            self.store.pop_front();
            return true;
        }
        
        false
    }
    
    fn front(&self) -> i32 {
        if self.store.len() > 0{
            *self.store.front().unwrap()
        }else{
            -1
        }
    }
    
    fn rear(&self) -> i32 {
        if self.store.len() > 0{
            *self.store.back().unwrap()
        }else{
            -1
        }
    }
    
    fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
    
    fn is_full(&self) -> bool {
        self.store.len() == self.capacity
    }
}
