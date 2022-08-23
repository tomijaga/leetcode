use std::collections::{BTreeMap};

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut freq = BTreeMap::new();
        let len = arr.len();
        
        for n in arr{
            *freq.entry(n).or_insert(0_i32) +=1;
        }
        
        let mut flipped_freq = freq.into_iter()
            .map(|(n, cnt)|{(cnt, n)})
            .collect::<Vec<(i32, i32)>>();
        
        flipped_freq.sort_unstable_by(|a, b| b.cmp(&a));
        
        let mut sum = 0;
        
        for (i, (cnt, n)) in flipped_freq.into_iter().enumerate(){
            // println!(": {:?}", (cnt, n));
            
            sum += cnt;
            
            if sum >= (len /2) as i32{
                return (i + 1) as i32;
            }
        }
        
        // println!("s: {:?}", sum);
        
        -1
    }
}