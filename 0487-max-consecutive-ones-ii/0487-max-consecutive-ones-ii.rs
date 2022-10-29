use std::collections::VecDeque;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut q : VecDeque<(i32, i32)> = VecDeque::new();
        let mut flipped = 0;
        let mut max_ones = 0;
        let mut cnt = 0;
                        //prev, after flipped zero
        let mut state = (0, 0);
        
        for n in nums{
            if n == 0{
                if flipped == 1{
                    state.1 = cnt - 1 - state.0;
                    let prev_state = state;
                    
                    if (prev_state != (0, 0)){
                        q.push_back(prev_state);
                    }
                    
                    state = (prev_state.1, 0);
                    cnt -= prev_state.0;
                }else{
                    flipped = 1;
                    state.0 = cnt;
                    cnt += 1;
                }
            }else{
                cnt+=1;
            }
            max_ones = max_ones.max(cnt);
        }
        
        // println!("{:?}", q);
        
        max_ones
    }
}