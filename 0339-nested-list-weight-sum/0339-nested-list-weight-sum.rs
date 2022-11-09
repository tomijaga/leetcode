// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }

use NestedInteger::*;

impl NestedInteger{
    fn depth_sum(&self, depth: i32) -> i32{
        match(self){
            NestedInteger::Int(n) => *n * depth,
            NestedInteger::List(arr) => {
                arr.into_iter()
                    .map(|item| item.depth_sum(depth + 1))
                    .sum()
            }
        }
    }
}

impl Solution {
    pub fn depth_sum(nested_list: Vec<NestedInteger>) -> i32 {
        NestedInteger::List(nested_list).depth_sum(0)
    }
}