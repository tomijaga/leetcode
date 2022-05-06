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

impl TreeNode{
    pub fn verify(&self, left_bound: Option<i32>, right_bound:Option<i32>) -> bool{
        let val = self.val;
        
        if let Some(l) = left_bound{
            if val <= l{
                return false;
            }
        }
        
        if let Some(r) = right_bound{
            if val >=r {
                return false;
            }
        }
        
        let mut res = true;
        
        if let Some(ref left) = &self.left{
            let is_valid = left.borrow().verify(left_bound, Some(val));
            
            res = res && is_valid;
        }
        
        if let Some(ref right) = &self.right{
            let is_valid = right.borrow().verify(Some(val), right_bound);
            
            res = res && is_valid;
        }
        
        res
    }
}

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root{
            node.borrow().verify(None, None)
        }else{
            true
        }
    }
}