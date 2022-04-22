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
use std::cmp;

impl TreeNode{
    pub fn maxDepth(&self) -> i32{
        fn _maxDepth(node: &TreeNode, max: &mut i32) -> i32{
            let mut left_h = 0;
            let mut right_h = 0;
        
            if let Some(ref left) = node.left{
                left_h =_maxDepth(&left.borrow(), max);
            }
        
            if let Some(ref right) = node.right{
                right_h = _maxDepth(&right.borrow(), max);
            }
            
            *max = cmp::max(*max, left_h+ right_h);
            
            cmp::max(left_h, right_h) + 1
        }
        
        let mut max = 0;
        let _ = _maxDepth(self, &mut max);
        max
    }
}

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root{
            node.borrow().maxDepth()
        }else{
            0
        }
    }
}