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
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() { return head; }
        
            let mut node = head.unwrap();
            let mut newHead = node.clone();
            let mut is_duplicate   = false;
            
            while let Some(next) = node.next{
                if node.val == next.val{
                    is_duplicate = true;
                    newHead = next.clone();
                }else{
                    break;
                }
                node = next;
            }
            
            return if is_duplicate{
                Solution::delete_duplicates(newHead.next)
            }else{
                newHead.next = Solution::delete_duplicates(newHead.next);
            
                Some(newHead)
            }
        
    }
}