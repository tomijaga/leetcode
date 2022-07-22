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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut secondGroup = vec![];
        
        fn to_list(secondGroup: &mut [i32]) -> Option<Box<ListNode>>{
            if secondGroup.len() > 0{
                let mut node = ListNode::new(secondGroup[0]);
                node.next = to_list(&mut secondGroup[1..]);
                
                Some(Box::new(node))
            }else{
                None
            }
        } 
        
        fn group(secondGroup: &mut Vec<i32>, node: &ListNode, x: i32) -> Option<Box<ListNode>>{
            if let Some(ref next) = node.next{
                if node.val < x{
                    let mut new_node = ListNode::new(node.val);
                    new_node.next = group(secondGroup, next, x);
                    
                    Some(Box::new(new_node))
                }else{
                    secondGroup.push(node.val);
                    
                    group(secondGroup, next, x)
                }
            }else{
                if node.val < x{
                    let mut new_node = ListNode::new(node.val);
                    new_node.next = to_list(secondGroup);
                    
                    Some(Box::new(new_node))
                }else{
                    secondGroup.push(node.val);
                    to_list(secondGroup)
                }
            }
        }
        
        
        if let Some(ref node) = head{
            group(&mut secondGroup, node, x)
        }else{
            None
        }
    }
}