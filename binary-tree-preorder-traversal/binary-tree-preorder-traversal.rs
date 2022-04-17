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

impl TreeNode {
    pub fn preorder_traversal(&self) -> Vec<i32> {
        
        fn traverse(node: &TreeNode, order: &mut Vec<i32>) {
            
            order.push(node.val);
            
            if let Some(ref left) = node.left {
                traverse(&left.borrow(), order);
            }

            if let Some(ref right) = node.right {
                traverse(&right.borrow(), order);
            }
        }

        let mut order = vec![];
        traverse(self, &mut order);
        order
    }
}

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        
        if let Some( node) = root{
            node.borrow().preorder_traversal()
        }else{
            vec![]
        }
        
    }
}