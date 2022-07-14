
use std::rc::Rc;
use std::cell::RefCell;

impl TreeNode{
    pub fn invert (&self)-> Self{

        let mut new_node = TreeNode::new(self.val);
        
        new_node.right = if let Some(ref left) = self.left{
            let left = left.borrow();
            Some(Rc::new(RefCell::new(left.invert())))
        }else{
            None
        };
        
        new_node.left = if let Some(ref right) = self.right{
            let right = right.borrow();
            Some(Rc::new(RefCell::new(right.invert())))
        }else{
            None
        };

        new_node
    }
}
impl Solution {
    pub fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref root) = root.clone(){
            let root = root.borrow();

            Some(Rc::new(RefCell::new(root.invert())))
        }else{
            None
        }
    }
}