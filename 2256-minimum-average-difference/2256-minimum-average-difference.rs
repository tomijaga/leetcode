
impl Solution {
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        
        let mut sum: i64 = 0;
        
        let sum_arr = nums.into_iter().map(|a|{
            sum += a as i64; 
            sum
        }).collect::<Vec<i64>>();
        
        let mut min = i64::MAX;
        let mut min_index = 0;
        
        for i in 0..sum_arr.len(){
            
            let sum_at_index = sum_arr[i];
            
            let first_avg =  sum_at_index/(i as i64 + 1);
            
            let last_divisor = if (i<sum_arr.len()-1) {
                sum_arr.len()  - i - 1
            } else {
                1
            } as i64;
            
            let last_avg = (sum - sum_at_index) / last_divisor;
        
            let avg = (first_avg - last_avg).abs();
                
            if (avg < min){
                min = avg;
                min_index = i;
            }
        }
        
        return min_index as i32;
    }
}