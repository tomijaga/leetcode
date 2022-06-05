impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut v = vec![];
        let mut total = 0;
        Self::backtrack(n, &mut v, &mut total);
        total
    }
    
    pub fn backtrack(n:i32, v: &mut Vec<i32>,  total: &mut i32){
        if v.len() == n as usize{
            *total+=1;
        }
        
        for i in 0..n{
            if v.iter()
                .enumerate()
                .any(|(j, &p)| p == i || (v.len() - j) as i32 == (p - i).abs())
            {
                continue;
            }
            v.push(i);
            Self::backtrack(n, v, total);
            v.pop();
        }
    }
}