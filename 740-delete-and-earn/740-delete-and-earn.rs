//  time complexity : O(n log n)
// space complexity : O(n)

use std::collections::BTreeMap;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut map = BTreeMap::new();
        
        for n in nums{
            *map.entry(n).or_insert(0)+= n;
        }
        
        let mut prev = (-1, 0);
        let mut twoPrev = prev;
        
        for (i, (&n, &total)) in map.iter().enumerate(){
            if i == 0{
                prev = (n, total);
                continue;
            }

            let tmp = prev;
            
            prev = if prev.0 + 1 == n{
                (n, (twoPrev.1 + total).max(prev.1))
            }else{
                (n, prev.1 + total)
            };
            
            twoPrev = tmp;
        }
        
        prev.1
    }
}