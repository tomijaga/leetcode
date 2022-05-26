// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;


impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() == 0{
            return None;
        }
        
        let max_index = nums
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)|{ a.cmp(&b) })
        .map(|(index, _)| index)
        .unwrap();
        
        let mut node = TreeNode::new(nums[max_index]);
        
        let left_sub = nums[..max_index].to_vec();
        let right_sub = nums[max_index + 1..].to_vec();
        
        node.left = Solution::construct_maximum_binary_tree(left_sub);
        node.right = Solution::construct_maximum_binary_tree(right_sub);
        
        Some(Rc::new(
            RefCell::new(
                node
            )
        ))
        
    }
}