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

pub fn construct(nums: &[i32])-> Option<Rc<RefCell<TreeNode>>>{
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
        
        let left_sub = &nums[..max_index];
        let right_sub = &nums[max_index + 1..];
        
        node.left = construct(left_sub);
        node.right = construct(right_sub);
        
        Some(Rc::new(
            RefCell::new(
                node
            )
        ))
        
}

impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        construct(&nums)
    }
}