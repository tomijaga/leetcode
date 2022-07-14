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
    pub fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref root) = root.clone(){
            let root = root.borrow();
            
            let mut new_node = TreeNode::new(root.val);
            new_node.left = Self::invert_tree(root.right.clone());
            new_node.right = Self::invert_tree(root.left.clone());
            
            Some(Rc::new(RefCell::new(new_node)))
        }else{
            None
        }
    }
}