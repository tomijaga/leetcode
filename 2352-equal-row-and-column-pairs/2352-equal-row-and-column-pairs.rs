use std::collections::HashMap;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut map = HashMap::new();
        
        let (m, n) = (grid.len(), grid[0].len());
        
        for row in grid.iter(){
            *map.entry(row).or_insert(0) += 1;
        }
        
        let mut pairs = 0;
        
        // println!("{:?}", &map);
        
        for j in 0..n{
            let col = (0..m).map(|i|{ grid[i][j] }).collect::<Vec<i32>>();
            
            if let Some(cnt) = map.get_mut(&col){
                if *cnt > 0{
                    pairs+=*cnt;
                }
            }
        }
        
        pairs
    }
}