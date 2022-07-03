use std::collections::HashMap;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        
        for n in nums{
            *map.entry(n).or_insert(0)+= n;
        }
    
        
        let mut arr = map.into_iter().collect::<Vec<_>>();
        
        if arr.len() == 1{
            return arr[0].1;
        }
        
        arr.sort_unstable_by_key(|a|a.0);
        
        // println!("{:?}", &arr);
        
        let mut memo = vec![0; arr.len()];
        memo[0] = arr[0].1;
        
        if arr[0].0 + 1 == arr[1].0{
            memo[1] = (arr[1].1).max(memo[0]);
        }else{
            memo[1] = memo[0] + arr[1].1;
        }
        
        for i in 2..arr.len(){
            if arr[i- 1].0 + 1 == arr[i].0{
                memo[i] = (memo[i - 2] + arr[i].1).max(memo[i - 1]);
            }else{
                memo[i] = memo[i - 1] + arr[i].1;
            }
        }
        
        // println!("{:?}", &memo);
        
        memo[arr.len() - 1].max(memo[arr.len() - 2])
    }
}