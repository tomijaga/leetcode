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

impl ListNode{
    pub fn len(&self) -> usize {
        if let Some(node) = self.next.clone(){
            if let Some(next) = node.next.clone(){
                next.len() + 2
            }else{
                node.len() + 1
            }
        }else{
            1
        }
    }
}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if let Some(node) = head{
           let len = node.len() as i32;
            
            Self::remove_from_end(Some(node), len - n)
        }else{
            None
        }
    }
    
    pub fn remove_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
       if let Some(mut node) = head{
           if n == 0{
               node.next
           }else{
               node.next = Self::remove_from_end(node.next, n - 1);
               Some(node)
           }
        }else{
            None
        }
    }
}