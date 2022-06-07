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
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut v = Self::list_to_vec(head.clone());

        for i in (0..v.len()).rev(){
            let mut sum = 0;
            for j in (i..v.len()){
                sum +=v[j];
                // println!("{}:{} sum: {:?}",i, j, sum);
                if sum == 0{
                    for k in (i..=j).rev(){
                        v.remove(k);
                    }
                    break;
                    
                }
            }
        }
        Self::vec_to_list(v)
    }
    
    pub fn list_to_vec(head: Option<Box<ListNode>>)-> Vec<i32>{
        let mut head = head;
        let mut vec = vec![];
        
        while let Some(node) = head{
            vec.push(node.val);
            head = node.next;
        }
        
        vec
    }
    
    
    pub fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        fn to_list(vec: &Vec<i32>, i: usize) ->  Option<Box<ListNode>> {
            if i < vec.len(){
                let mut node = ListNode::new(vec[i]);
                node.next = to_list(vec, i+ 1);
                Some(Box::new(node))
            }else{
                None
            }
        }
        
        to_list(&vec, 0)
    }
}