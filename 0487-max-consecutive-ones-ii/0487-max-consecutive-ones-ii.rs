impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut flipped = false;
        let mut max_ones = 0;
        let mut cnt = 0;
                        //prev, after flipped zero
        let mut state = (0, 0);
        
        for n in nums{
            if n == 0{
                if flipped{
                    state.1 = cnt - state.0 - 1;
                    let prev_state = state;
                    
                    state = (prev_state.1, 0);
                    cnt -= prev_state.0;
                }else{
                    flipped = true;
                    state.0 = cnt;
                    cnt += 1;
                }
            }else{
                cnt+=1;
            }
            
            max_ones = max_ones.max(cnt);
        }
        
        max_ones
    }
}