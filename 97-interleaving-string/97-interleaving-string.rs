impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        
        let s3: Vec<char> = s3.chars().collect();
        
        let mut memo = vec![vec![-1; s2.len() + 1]; s1.len() + 1];
        
        interleave(&s3, &s1, &s2, &mut memo)
    }
}

pub fn interleave(s3: &[char], s1: &[char], s2: &[char], memo: &mut Vec<Vec<i32>>) -> bool {
    if s3.len() == 0{
        if  s2.len() == 0 && s1.len() == 0{
            return true;
        }
        
        return false;
    }
    
    let target = s3[0];
    
    let memo_val = memo[s1.len()][s2.len()];
    if memo_val >-1{
        return if (memo_val == 1){true} else {false};
    }
    
    let mut res = false;
    
    if let Some(&a) = s1.first(){
        if let Some(&b) = s2.first(){
            if a == b && b == target{
                res |=interleave(&s3[1..], &s1[1..], s2, memo);
                
                res |=interleave(&s3[1..], s1, &s2[1..], memo);
                
            }else if a == target{
                res |=interleave(&s3[1..], &s1[1..], s2, memo);
            }else if b == target{
                res |=interleave(&s3[1..], s1, &s2[1..], memo);
            }
        }else{
            if a == target{
                res |=interleave(&s3[1..], &s1[1..], s2, memo);
            }
        }
    }else if let Some(&b) = s2.first(){
        if b == target{
            res |=interleave(&s3[1..], s1, &s2[1..], memo);
        }
    }
    
    memo[s1.len()][s2.len()] = if res {1} else {0};
    
    res
}