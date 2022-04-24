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

pub fn new_node(val: i32) -> Option<Box<ListNode>>{
    Some(
        Box::new(
            ListNode::new(val)
        )
    )
}

pub fn vec_to_list_node(vec: &mut Vec<&Box<ListNode>>, i: usize) -> Option<Box<ListNode>>{
    if (i >= vec.len()){
        return None;
    }
    
    let mut node = vec[i].clone();
    node.next = vec_to_list_node(vec, i+1);
     
    return Some(node);
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.len() == 0 {
            return None;
        }
        
        let mut heap = BinaryHeap::new();
        
        for mut list in lists.iter(){
            while let Some(_list) = list {
                list = &_list.next;
                heap.push(_list);
                
            }
        }
        
        let mut heap_vec = heap.into_sorted_vec();
        vec_to_list_node(&mut heap_vec, 0)
    }
}