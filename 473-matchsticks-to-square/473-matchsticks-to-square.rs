impl Solution {
    pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
        if matchsticks.len() == 0{
            return false;
        }
        
        let sum: i32 = matchsticks.iter().sum();
        let side_len = sum / 4;
        
        if side_len * 4 != sum{
            return false;
        }
        
        matchsticks.sort_unstable_by(|a, b| b.cmp(&a));
        
        let mut sides = vec![0; 4];
        
        dfs(&matchsticks, &mut sides, side_len, 0)
    }
}

pub fn dfs(matchsticks: &Vec<i32>, sides: &mut Vec<i32>, side_len: i32, index: usize) -> bool{
    if index + 1 == matchsticks.len() {
        return  sides[0..3].into_iter().all(|&side|{ side == side_len});
    }
    
    for i in 0..4{
        if sides[i] + matchsticks[index] <= side_len{
            sides[i] += matchsticks[index];
            if dfs(matchsticks, sides, side_len, index + 1){
                return true;
            }
            
            sides[i] -= matchsticks[index];
        }
    }
    
    false
}