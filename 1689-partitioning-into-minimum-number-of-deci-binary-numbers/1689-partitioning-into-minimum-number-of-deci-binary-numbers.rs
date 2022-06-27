impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.chars().max().unwrap().to_string().parse::<i32>().unwrap()
    }
}