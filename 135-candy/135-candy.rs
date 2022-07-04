impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut ans = vec![1; n];
        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                ans[i] = ans[i - 1] + 1;
            }
        }
        for i in (0..(n - 1)).rev() {
            if ratings[i] > ratings[i + 1] {
                ans[i] = std::cmp::max(ans[i + 1] + 1, ans[i]);
            }
        }
        ans.into_iter().sum()
    }
}