use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::min;

impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        
        pub fn cnt_camera(node: &TreeNode, res: &mut i32) -> i32 {
            let (mut left_cnt, mut right_cnt) = (0, 0);
            let has_child = false;
            
            let l = if let Some(ref left) = node.left{
                let l = left.borrow();
                cnt_camera(&l, res)
            }else{
                2
            };
            
            let r = if let Some(ref right) = node.right{
                let r = right.borrow();
                cnt_camera(&r, res)
            }else{
                2
            };
            
            
            if l == 0 || r == 0{
                *res +=1;
                1
            }else{
                if (l == 1 || r == 1) {
                    2
                }else{
                    0
                }
            }
        }
        
        
        let mut res = 0;
        if let Some(ref node) = root {
            let node_msg = cnt_camera(&node.borrow(), &mut res);
            (if node_msg == 0{
                1
            }else{
                0
            }) + res
        }else{
            0
        }
        
    }
}