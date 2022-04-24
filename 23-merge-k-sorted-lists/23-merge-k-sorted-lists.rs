// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

use std::collections::BinaryHeap;
use std::cmp::{Reverse, Ord, Ordering};

impl PartialOrd for ListNode{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        Some(self.val.cmp(&other.val))
    }
}

impl Ord for ListNode{
    fn cmp(&self, other: &Self) -> Ordering{
        self.val.cmp(&other.val)
    }
}

pub fn new_node(val: i32) -> Box<ListNode> {
        Box::new(
            ListNode::new(val)
        )
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.len() == 0 {
            return None;
        }
        
        let mut heap = BinaryHeap::new();
        
        for mut list in lists{
            if let Some(l)= list{
                heap.push(Reverse(l));
            }
        }
        
        let mut dummy = new_node(-1);
        let mut curr = &mut dummy;
        
        while let Some(Reverse(node)) = heap.pop() {
            curr.next = Some(node.clone());
            curr = curr.next.as_mut().unwrap();
            
            if let Some(next) = node.next {
                heap.push(Reverse(next));
            }
            
        }
        
        dummy.next
    }
}