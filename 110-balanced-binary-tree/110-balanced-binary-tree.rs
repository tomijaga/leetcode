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
use std::cmp::max;

impl TreeNode{
    pub fn get_balanced_height(&self)-> i32{
        let mut lh = 0;
        let mut rh = 0;
        
        if let Some(ref left) = self.left{
            lh = left.borrow().get_balanced_height();
        }
        
        if let Some(ref right) = self.right{
            rh = right.borrow().get_balanced_height();
        }
        
        if lh == -1 || rh == -1 {
            -1
        }else if (lh - rh).abs() > 1 {
            -1
        }else{
            max(rh, lh)  + 1
        }
    }
}

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(ref node) = root {
            let h = node.borrow().get_balanced_height();
            // println!("h: {}", h);÷
            h != -1
        }else{
            true
        }
    }
}