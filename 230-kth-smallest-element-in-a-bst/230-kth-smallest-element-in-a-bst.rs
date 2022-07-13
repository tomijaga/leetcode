
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
        let mut heap = BinaryHeap::new();
        
        fn trav(node: &TreeNode, heap: &mut BinaryHeap<i32>, k: usize){
            heap.push(node.val);
            
            if heap.len() > k {
                heap.pop();
            }
            
            if let Some(ref l) = node.left{
                let l = l.borrow();
                trav(&l, heap, k)
            }
            
            if let Some(ref r) = node.right{
                let r = r.borrow();
                trav(&r, heap, k)
            }
        }
        
        trav(self, &mut heap, k as usize);
        
        *heap.peek().unwrap()
    }
}