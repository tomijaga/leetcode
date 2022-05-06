use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut map = HashMap::new();
        
        for n in nums{
            if let Some(count) = map.get_mut(&n){
                *count +=1;
                if *count > l/2{
                    return n;
                }
            }else{
                map.insert(n, 1);
            }
        }
        
        map.iter().fold((0, 0), |(maxn, max), (&n, &count)|{
            if count > max{
                (n, count)
            }else{
                (maxn, max)
            }
        }).0
    }
}