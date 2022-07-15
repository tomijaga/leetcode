impl Solution {
    pub fn shortest_distance_color(colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut shortest:Vec<Vec<usize>> = Vec::with_capacity(colors.len());
        
        let mut prev = vec![usize::MAX, usize::MAX, usize::MAX];
        
        for (i, &c) in colors.iter().enumerate(){
            let c = c as usize - 1; 
            prev[c]  = i;
            
            let diff = prev.iter().map(|&prev_index| {
                if prev_index == usize::MAX{
                    prev_index
                }else{
                    i - prev_index
                }
            }).collect();
            
            shortest.push(diff);
        }
        
        prev = vec![usize::MAX, usize::MAX, usize::MAX];
        
        for (i, c) in colors.into_iter().enumerate().rev(){
            let c = c as usize - 1; 
            
            prev[c]  = i;
            
            shortest[i] = prev.iter().enumerate().map(|(c, &prev_index)| {
                (if prev_index == usize::MAX{
                    prev_index
                }else{
                    prev_index - i
                })
                .min(shortest[i][c])
            }).collect();
            
        }
        
        let mut res = Vec::with_capacity(queries.len());
        
        for query in queries{
            let i = query[0] as usize;
            let c = query[1] as usize - 1; 
            
            let mut short = &shortest[i];
            
            if short[c] == usize::MAX{
                res.push(-1);
            }else{
                res.push( short[c] as i32);
            }
        }
        
        res
    }
}