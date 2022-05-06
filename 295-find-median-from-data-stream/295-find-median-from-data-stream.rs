struct MedianFinder {
    nums: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        Self{
            nums: vec![]
        }
    }
    
    fn add_num(&mut self, num: i32) {
        self.nums.push(num);
    }
    
    fn find_median(&mut self) -> f64 {
        self.nums.sort_unstable();
        
        let len = self.nums.len();
        let mid = len/2;
        
        if len % 2 == 0{
            (self.nums[mid] + self.nums[mid-1]) as f64 / 2_f64
        }else{
            self.nums[mid] as f64
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */