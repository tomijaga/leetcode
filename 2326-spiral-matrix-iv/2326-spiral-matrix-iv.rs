impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let (m, n) = (m as usize, n as usize);
        let mut matrix = vec![vec![-1; n]; m];
        
        let (mut top, mut bottom, mut left, mut right) = (0, m, 0, n);
        
        let mut dummy = ListNode::new(-1);
        dummy.next = head;
        let mut curr = &dummy;
        let mut filled = 0;
        
        while curr.next.is_some() && filled < (n*m) {
            
            for j in left..right{
                
                if let Some(ref node) = curr.next{
                    matrix[top][j] = node.val;
                    curr = node;
                    filled+=1;
                }
            }
            
            top+=1;
            
            for i in top..bottom{
                
                if let Some(ref node) = curr.next{
                    matrix[i][right -1] = node.val;
                    curr = node;
                    filled+=1;
                }
            }
            
            right -=1;
            
            for j in (left..right).rev(){
                
                if let Some(ref node) = curr.next{
                    matrix[bottom -1][j] = node.val;
                    curr = node;
                    filled+=1;
                }
            }
            
            bottom-=1;
            
            for i in (top..bottom).rev(){
                
                if let Some(ref node) = curr.next{
                    matrix[i][left] = node.val;
                    curr = node;
                    filled+=1;
                }
            }
            
            left +=1;
            
        }
        
        matrix
    }
}