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
    pub fn post_trav(&self) ->Vec<i32>{
        fn traverse(node: &TreeNode, stack:&mut Vec<i32>){
            if let Some(ref left)  = node.left{
                traverse(&left.borrow(), stack);
            }
            
            if let Some(ref right)  = node.right{
                traverse(&right.borrow(), stack);
            }
            
            stack.push(node.val);
        }
        
        let mut stack = vec![];
        traverse(self, &mut stack);
        stack
    }
}
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some(node) = root{
            node.borrow().post_trav()
        }else{
            vec![]
        }
    }
}