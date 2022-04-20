
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 

use std::rc::Rc;
use std::cell::RefCell;

impl TreeNode {
  #[inline]
  pub fn inorder_traversal(&self) -> Vec<i32> {
      
      fn trav (node: &TreeNode, stack: &mut Vec<i32>){
          if let Some(ref left) = node.left{
               trav(&left.borrow(), stack);
          }
          
          stack.push( node.val);
          
          if let Some(ref right) = node.right{
               trav(&right.borrow(), stack);
          }
      }
      
      let mut stack = vec![];
     
      trav(self, &mut stack);
      stack.reverse();
      stack
  }
}

struct BSTIterator {
    stack: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = vec![];
        if let Some(node) = root{
            let in_order_stack = node.borrow().inorder_traversal();
            stack = in_order_stack;
        }
        
        // println!("{:?}", stack);
        BSTIterator{
            stack: stack,
        }
    }
    
    fn next(&mut self) -> i32 {
        
        if let Some(val) = self.stack.pop(){
            return val;
        }
        unreachable!();
    }
    
    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */