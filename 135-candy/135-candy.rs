impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut res = vec![1; n];
        
        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                res[i] = res[i - 1] + 1;
            }
        }
        
        for i in (0..ratings.len() -1).rev() {
            if ratings[i] > ratings[i + 1] {
                res[i] = (res[i + 1] + 1).max(res[i]);
            }
        }
        
        res.into_iter().sum()
    }
}