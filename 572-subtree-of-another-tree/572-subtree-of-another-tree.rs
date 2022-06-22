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
        
        fn both_match(node: &Option<Rc<RefCell<TreeNode>>>, sub: &Option<Rc<RefCell<TreeNode>>>)-> bool{
            if node.is_some() && sub.is_some(){
                
                let un_node = &node.clone().unwrap();
                let un_sub = &sub.clone().unwrap();
                
                let node = un_node.borrow();
                let sub = un_sub.borrow();
                
                
                if node.val == sub.val{
                    both_match(&node.left, &sub.left) &&
                    both_match(&node.right, &sub.right)
                }else{
                    false
                }
                
            }else if node.is_none() && sub.is_none(){
                true
            }else{
                false
            }
        }
        
        fn has_a_match(node: &Option<Rc<RefCell<TreeNode>>>, sub: &Option<Rc<RefCell<TreeNode>>>, has_match: &mut bool){
            if both_match(node, sub){
                *has_match = true;
                return;
            }else{
                if let Some(ref n) = node{
                    let n = n.borrow();
                    
                    has_a_match(&n.left, sub, has_match);
                    has_a_match(&n.right, sub, has_match);
                }
            }
        }
        
        let mut has_match = false;
        
        has_a_match(&root, &sub_root, &mut has_match);
        has_match
    }
}