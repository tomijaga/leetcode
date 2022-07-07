impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let mut s1: Vec<char> = s1.chars().rev().collect();
        let mut s2: Vec<char> = s2.chars().rev().collect();
        
        let mut s3: Vec<char> = s3.chars().collect();
        
        let mut memo = vec![vec![-1; s2.len() + 1]; s1.len() + 1];
        
        interleave(&s3, &mut s1, &mut s2, &mut memo)
    }
}

pub fn interleave(s3: &[char], s1: &mut Vec<char>, s2: &mut Vec<char>, memo: &mut Vec<Vec<i32>>) -> bool {
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
    
    if let Some(&a) = s1.last(){
        if let Some(&b) = s2.last(){
            if a == b && b == target{
                s2.pop();
                res |=interleave(&s3[1..], s1, s2, memo);
                s2.push(b);
                
                s1.pop();
                res |=interleave(&s3[1..], s1, s2, memo);
                s1.push(a);
                
            }else if a == target{
                s1.pop();
                res |=interleave(&s3[1..], s1, s2, memo);
                s1.push(a);
            }else if b == target{
                s2.pop();
                res |=interleave(&s3[1..], s1, s2, memo);
                s2.push(b);
            }
        }else{
            if a == target{
                s1.pop();
                res |=interleave(&s3[1..], s1, s2, memo);
                s1.push(a);
            }
        }
    }else if let Some(&b) = s2.last(){
        if b == target{
            s2.pop();
            res |=interleave(&s3[1..], s1, s2, memo);
            s2.push(b);
        }
    }
    
    memo[s1.len()][s2.len()] = if res {1} else {0};
    
    res
}