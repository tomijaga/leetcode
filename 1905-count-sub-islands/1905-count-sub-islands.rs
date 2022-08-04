pub const DIR: [(i32, i32); 4] = [(1, 0), ( -1, 0), (0, 1), (0, -1)];

impl Solution {
    pub fn count_sub_islands(mut grid1: Vec<Vec<i32>>, mut grid2: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid1.len(), grid1[0].len());
        let mut cnt = 0;
        
        for i in 0..m{
            for j in 0..n{
                if dfs(&mut grid1, &mut grid2, i, j){
                    cnt +=1;
                }
            }
        }
        
        cnt
    }
}

fn dfs(g1: &mut Vec<Vec<i32>>, g2: &mut Vec<Vec<i32>>, i: usize, j: usize) -> bool {
    let (m, n) = (g1.len(), g1[0].len());
    
    if g1[i][j] != 1 || g2[i][j] != 1{
        return false;
    }
    
    g2[i][j] = 2;
    
    let mut res = true;
    
    for (dx, dy) in DIR{
        let i = (dx + i as i32) as usize;
        let j = (dy + j as i32) as usize;
        
        if (i != usize::MAX && j != usize::MAX && i < m && j < n && g2[i][j] == 1){
            res &= dfs(g1, g2, i, j);
            
        }
    };
    
    res
}