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

type OptTreeNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_subtree(root: OptTreeNode, sub_root: OptTreeNode) -> bool {
        
        fn has_match(root: &OptTreeNode, sub: &OptTreeNode) -> bool{
            if (root == sub){
                true
            }else{
                if let Some(ref node) = root{
                    let node = node.borrow();
                    
                    has_match(&node.left, &sub) || 
                    has_match(&node.right, &sub)
                }else{
                    false
                }
            }
        }
        
        has_match(&root, &sub_root)
    }
}