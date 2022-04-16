use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        
        let (small, large) = if (nums1.len()< nums2.len()) {(nums1, nums2)} else {(nums2, nums1)};
            
        for n in small{
            if let Some(times) = map.get_mut(&n){
                *times+=1;
            }else{
                map.insert(n, 1);
            }
        }
        
        println!("map: {:?}", map);
        let mut intersection = vec![];
        
        for n in large{
            if let Some(times) = map.get_mut(&n){
                intersection.push(n);
                *times-=1;
                
                if (*times == 0){
                    map.remove(&n);
                }
                println!("n: {}, map: {:?}",n,   map);
            }
        }
        
        return intersection;
        
    }
}