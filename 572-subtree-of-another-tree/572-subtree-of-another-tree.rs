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
type OptRefTreeNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        
        if (root == sub_root){
            true
        }else{
            if let Some(ref node) = root{
                let node = node.borrow();
                
                Self::is_subtree(node.left.clone(), sub_root.clone()) || 
                Self::is_subtree(node.right.clone(), sub_root.clone())
            }else{
                false
            }
        }
        
    }
}