impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut arr = vec![vec![0_i32;candidates.len()];32];
        
        for (i, n) in candidates.into_iter().enumerate(){
            for (j, b) in format!("{:#034b}", n).chars().skip(2).enumerate(){
                arr[j][i] = if b == '0' {0} else{1};
            }
        }
        
        arr.into_iter()
            .map(|v| {
                v.into_iter().sum::<i32>()
            })
            .max()
            .unwrap()
    }
}