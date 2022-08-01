struct NumArray {
    nums: Vec<i32>
}


impl NumArray {

    fn new(mut nums: Vec<i32>) -> Self {
        nums.reverse();
        nums.push(0);
        nums.reverse();
        
        for i in (1..nums.len()){
            nums[i]+= nums[i - 1];
        }
        println!("{:?}", &nums);
        Self{ nums }
    }
    
    fn get(&self, index: usize) -> i32{
        self.nums[index + 1]
    }
    
    fn get_num(&self, index: usize) -> i32{
        self.nums[index + 1] - self.nums[index]
    }
    
    fn update(&mut self, index: i32, val: i32) {
        let index = index as usize;
        
        let diff = val - self.get_num(index);
        
        for i in index + 1..self.nums.len(){
            self.nums[i] += diff;
        }
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.nums[(right + 1) as usize] - self.nums[left as usize]
    }
}
