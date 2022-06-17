use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::min;

#[derive(Eq, PartialEq)]
pub enum NodeMsg{
    Leaf,
    Camera,
    Skip    
}

impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        
        pub fn cnt_camera(node: &TreeNode, res: &mut i32) -> NodeMsg {
            let l = if let Some(ref left) = node.left{
                let l = left.borrow();
                cnt_camera(&l, res)
            }else{
                NodeMsg::Skip
            };
            
            let r = if let Some(ref right) = node.right{
                let r = right.borrow();
                cnt_camera(&r, res)
            }else{
                NodeMsg::Skip
            };
            
            if l == NodeMsg::Leaf || r == NodeMsg::Leaf{
                *res +=1;
                NodeMsg::Camera
                
            }else if l == NodeMsg::Camera || r == NodeMsg::Camera {
                NodeMsg::Skip
                
            }else{
                NodeMsg::Leaf
            }
        }
        
        let mut res = 0;
        if let Some(ref node) = root {
            let node_msg = cnt_camera(&node.borrow(), &mut res);
            
            if node_msg == NodeMsg::Leaf{
                1 + res
            }else{
                res
            }
            
        }else{
            0
        }
        
    }
}