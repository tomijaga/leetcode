use std::collections::BTreeMap;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut map = BTreeMap::new();
        
        for n in nums{
            *map.entry(n).or_insert(0)+= n;
        }
    
        
        let mut memo = vec![0; map.len()];
        let mut twoPrev = -1;
        let mut prev = -1;
        let mut max = 0;
        
        for (i, (&n, &total)) in map.clone().iter().enumerate(){
            if i == 0{
                prev = n;
                continue;
            }
            
            if prev + 1 == n{
                let one = *map.get(&prev).unwrap();
                let two = if twoPrev > 0{
                    *map.get(&twoPrev).unwrap()
                }else{
                    0
                };
                
                map.insert(n, (two + total).max(one));
            }else{
                let one = *map.get(&prev).unwrap();
                map.insert(n, one + total);
            }
            
            twoPrev = prev;
            prev = n;
        }
        
        // println!("{:?}", &map);
        
        *map.get(&prev).unwrap()
    }
}