impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        
        println!("{:?}", &nums);
        let mut res = vec![];
        
        for i in 0..(nums.len() as i32 - 2) {
            let i = i as usize;
            
            if i > 0 && nums[i - 1] == nums[i]{
                continue;
            }
            
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            
            while j < k{
                let sum = nums[i] + nums[j] + nums[k];
                // println!("{:?}, {}", [nums[i], nums[j], nums[k]], sum );
                
                if sum > 0{
                    k-=1;
                }else if sum < 0{
                    j +=1;
                }else{
                    let v = vec![nums[i], nums[j], nums[k]];
                    res.push(v);
                    
                    while j < k && nums[j] == nums[j + 1]{
                        j +=1;
                    }
                    
                    while j < k && nums[k] == nums[k - 1]{
                        k-=1;
                    }
                    
                    j+=1;
                    k-=1;
                }
                
            }
            
        }
        
        res
    }
}