impl Solution {
    pub fn shortest_distance_color(colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut shortest:Vec<Vec<usize>> = vec![];
        
        let mut prev = vec![usize::MAX, usize::MAX, usize::MAX];
        let len = colors.len();
        
        for (i, &c) in colors.iter().enumerate(){
            let c = c as usize - 1; 
            prev[c]  = i;
            
            let curr = prev.iter().map(|&index| {
                if index == usize::MAX{
                    index
                }else{
                    i - index
                }
            }).collect();
            
            shortest.push(curr);
        }
        
        prev = vec![usize::MAX, usize::MAX, usize::MAX];
        
        let mut len = colors.len();
        // println!("s: {:?}", &shortest);
        for (i, c) in colors.into_iter().enumerate().rev(){
            let c = c as usize - 1; 
            
            prev[c]  = i;
            
            shortest[i] = prev.iter().enumerate().map(|(c, &index)| {
                (if index == usize::MAX{
                    index
                }else{
                    index - i
                })
                .min(shortest[i][c])
            }).collect();
            
        }
        
        // println!("s: {:?}", &shortest);
        
        let mut res = vec![];
        
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