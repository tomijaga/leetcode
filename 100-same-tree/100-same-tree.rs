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
use std::cmp::Ordering;

type TreeRefNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn cmp_trees(p:&TreeRefNode, q: &TreeRefNode ) -> bool {
        
        match (p, q) {
            (Some(ref p), Some(ref q)) => {
                let p = p.borrow();
                let q = q.borrow();
                
                let left_order = Solution::cmp_trees(&p.left, &q.left);
                let right_order = Solution::cmp_trees(&p.right, &q.right);
                
                return p.val == q.val && left_order && right_order;
            },
            (Some(_), None) => false,
            (None, Some(_)) => false,
            (_, _) => true
        }
        
    }
    
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::cmp_trees(&p, &q)
    }
}