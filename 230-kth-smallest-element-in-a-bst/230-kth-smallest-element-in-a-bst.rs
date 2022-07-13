
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BinaryHeap;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        root.unwrap().borrow().kth_smallest(k)
    }
}

impl TreeNode{
    pub fn kth_smallest(&self, k: i32)-> i32{
        let mut v = vec![];
        
        fn trav(node: &TreeNode, v: &mut Vec<i32>, k: usize){
            if let Some(ref l) = node.left{
                let l = l.borrow();
                trav(&l, v, k)
            }
            
            if v.len() < k {
                v.push(node.val);
            }else{
                return;
            }
            
            if let Some(ref r) = node.right{
                let r = r.borrow();
                trav(&r, v, k)
            }
        }
        
        trav(self, &mut v, k as usize);
        
        v.pop().unwrap()
    }
}