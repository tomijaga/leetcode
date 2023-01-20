use std::collections::BTreeMap;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut max_vec = vec![];
        let k = k as usize;
        let mut map = BTreeMap::new();

        for &n in &nums[0..k]{
            *map.entry(n).or_insert(0) += 1;
        }
        
        max_vec.push(*(map.iter().rev().next().unwrap().0));

        for i in 0..(nums.len() - k){
            let j = i + k;

            let a = nums[i];
            let b = nums[j];

            if let Some(cnt) = map.get_mut(&a){
                *cnt -=1;
                if *cnt == 0{
                    map.remove(&a);
                }
            }

            *map.entry(b).or_insert(0) += 1;

            max_vec.push(*(map.iter().rev().next().unwrap().0));
        }

        max_vec
    }
}