impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        
        let mut stack = vec![];
        
        let (mut si, mut sj) = (0, 0);
        let mut islands = 0;
        
        while let Some((i, j))  = get_next_island(&grid, si, sj) {
            si = i;
            sj = j;
            islands+=1;
            stack.push((i, j));

            while !stack.is_empty(){
                let (i, j) = stack.pop().unwrap();
                
                if grid[i][j] == '1'{
                    grid[i][j] = '0';
                    
                    for (x, y) in [(i + 1, j), (i - 1, j), (i, j+ 1),(i, j-1)]{
                        if x!=usize::MAX && x < m && y!=usize::MAX && y < n && grid[x][y] == '1'{
                            stack.push((x, y));
                        }
                    }
                }
            }
        }
        
        islands
        
    }
}

fn get_next_island(grid: &Vec<Vec<char>>, startI: usize, startJ: usize) -> Option<(usize, usize)>{
    let (m, n) = (grid.len(), grid[0].len());
    
    if startI == 0 && startJ == 0 && grid[startI][startJ] == '1'{
        return Some((0, 0));
    }
    
    for i in startI..m{
        for j in 0..n{
            if (i == startI && j > startJ || i > startI) && grid[i][j] == '1'{
                return Some((i, j));
            }
        }
    }
    
    None
}