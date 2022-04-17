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
    pub fn inorder_traversal(&self) -> Vec<i32>{
        let mut stack: Vec<i32> = vec![];
        
        fn traverse(node: &TreeNode, stack: &mut Vec<i32>){
            if let Some(ref left) = node.left{
                traverse(&left.borrow(), stack);
            }
            
            stack.push(node.val);
            
            if let Some(ref right) = node.right{
                traverse(&right.borrow(), stack);
            }
        }
        
        traverse(self, &mut stack);
        stack
    }
}
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some( node ) = root {
            node.borrow().inorder_traversal()
        }else{
            vec![]
        }
    }
}