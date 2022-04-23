
pub fn swap(nums: &mut Vec<i32>, i: usize, j: usize){
    let temp = nums[i];
    nums[i] = nums[j];
    nums[j] = temp;
}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut count = 0;
        let mut curr = nums[0];
        
        let l = nums.len();
        let mut  i = 0;
        let mut skip = 0;
        let mut unique = 0;
        
        while(i<l){
            

            
            let n = &mut nums[i];
            
            if (curr == *n){
                count+=1;
            }else{
                count  = 1;
                curr = *n;
                
            }
            
            // println!("count: {}, n:{}, skip: {}, u :{}", count, *n, skip, unique);
            
            if (count > 2){
                    *n = 0;
                    skip+=1;
            }else{
                if (skip >  0){
                    // println!("a: {:?}", nums);
                    swap(nums, i, i-skip);
                    // println!("b: {:?}", nums);
                }
            }
            
            i+=1;
        }
        
        (l - skip) as i32
    }
}