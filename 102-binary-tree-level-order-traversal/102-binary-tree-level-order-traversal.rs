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
use std::collections::VecDeque;

impl Clone for TreeNode {
    fn clone(&self) -> TreeNode {
        let mut t = TreeNode::new(self.val);
        t.left = self.left.clone();
        t.right = self.right.clone();
        t
    }
}

impl TreeNode{
    pub fn lvl(&self) -> Vec<Vec<i32>>{
        
        let mut deque: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::from([Some(
            Rc::new(RefCell::new(self.clone()))
            )]);
        let mut order:Vec<Vec<i32>> = vec![];
        let mut stack = vec![];
        
        while(!deque.is_empty()){
            for _ in 0..deque.len(){
                if let Some(node) = deque.pop_front().unwrap(){
                    
                    deque.push_back(node.borrow().left.clone());
                    deque.push_back(node.borrow().right.clone());
                    stack.push(node.borrow().val);
                }
            }
            if stack.len() > 0{
                order.push(stack.clone());
                stack.clear();
            }
        }

        order
    }
}

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if let Some(node) = root{
            node.borrow().lvl()
        }else{
            vec![]
        }
    }
}