impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() < 2{
            return nums.len() as i32;
        }
        
        let mut max = 1;
        let mut peak = false;
        let mut first = true;
        
        for i in 1..nums.len(){
            if nums[i] < nums[i - 1] && (peak == true || first == true){
                // println!("peak", );
                
                peak = false;
                max+=1;
                first = false;
                
            }else if nums[i] > nums[i-1] && (peak == false || first == true){
                // println!("valley");
                peak = true;
                max += 1;
                first = false;
                
            }
        }
        
        max
    }
}