
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
          
          stack.push(node.val);
          
          if let Some(ref right) = node.right{
              trav(&right.borrow(), stack);
          }
      }
      
      let mut stack = vec![];
     
      trav(self, &mut stack);
      stack
  }
}

struct BSTIterator {
    array: Vec<i32>,
    index: usize
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut array = vec![];
        if let Some(node) = root{
            let in_order_stack = node.borrow().inorder_traversal();
            array = in_order_stack;
        }
        
        // println!("{:?}", array);
        BSTIterator{
            array: array,
            index: 0
        }
    }
    
    fn next(&mut self) -> i32 {
        let prev  = self.index;
        self.index+=1;
        
        self.array[prev]
    }
    
    fn has_next(&self) -> bool {
        self.index < self.array.len()
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */