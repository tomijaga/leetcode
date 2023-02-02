#[derive(Default)]
struct MyCircularQueue {
    start: usize,
    count: usize,
    storage: Vec<i32>
}

impl MyCircularQueue {

    fn new(k: i32) -> Self {
        Self { 
            storage: vec![0; k as usize],
            ..Default::default()
        }
    }
    
    fn get_index(&self, i: usize) -> usize{
        (self.start + i) % self.storage.capacity()
    }
    
    fn en_queue(&mut self, value: i32) -> bool {
        if (self.count == self.storage.capacity()){
            return false;
        }
        
        let index = self.get_index(self.count);
        self.storage[index] = value;
        self.count += 1;
        true
    }
    
    fn de_queue(&mut self) -> bool {
        if (self.count == 0){
            return false;
        }
        
        self.count -= 1;
        self.start = self.get_index(1);
        
        return true;
    }
    
    fn front(&self) -> i32 {
        if (self.count == 0){
            -1
        }else{
            self.storage[self.start]
        }
    }
    
    fn rear(&self) -> i32 {
        if (self.count == 0){
            return -1;
        }
        
        let index = self.get_index(self.count - 1);
        self.storage[index]
    }
    
    fn is_empty(&self) -> bool {
        self.count == 0
    }
    
    fn is_full(&self) -> bool {
        self.count == self.storage.capacity()   
    }
}