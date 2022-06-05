
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let median = nums.len()/2;
        let n = nums[median];
        let mut node = TreeNode::new(n);
        
        let ls = nums[..median].to_vec();
        let rs = nums[median + 1..].to_vec();
        
        Self::merge(&mut node, ls);
        Self::merge(&mut node, rs);
        
        Self::wrap(node)
        
    }
    
    pub fn merge(node: &mut TreeNode,  nums: Vec<i32>){
        if (nums.len() <= 0){
            return;
        }
        
        let median_index = nums.len()/2;;
        
        let median = nums[median_index];
        
        let mut child = TreeNode::new(median);
        
        let ls = nums[..median_index].to_vec();
        let rs = nums[median_index + 1..].to_vec();
        
        Self::merge(&mut child, ls);
        Self::merge(&mut child, rs);
        
        let wrapped_child = Self::wrap(child);
        
        if median > node.val{
            node.right = wrapped_child
        }else{
            node.left = wrapped_child
        }
    }
    
    pub fn wrap(node: TreeNode) -> Option<Rc<RefCell<TreeNode>>>{
            Some(
                Rc::new(
                    RefCell::new(node)
                )
            )
    }
}