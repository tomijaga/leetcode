impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (m,n) = (board.len(), board[0].len());
        
        for dir in [1, -1]{
            for i in 0..m{
                if dir == 1{
                    dfs(board, i, 0);
                }else{
                    dfs(board, m-i -1, n - 1);
                }
            }
            
            for j in 0..n{
                if dir == 1{
                    dfs(board, 0, j);
                }else{
                    dfs(board, m-1, n - j -1);
                }
            }
        }
        
        // println!("{:?}", &board);
        
        for i in 0..m{
            for j in 0..n{
                if board[i][j] == '#'{
                    board[i][j] = 'O';
                }else if board[i][j] == 'O'{
                    board[i][j] = 'X';
                }
            }
        }
    }
}

pub fn dfs(board: &mut Vec<Vec<char>>, i: usize, j: usize){
    let (m,n) = (board.len(), board[0].len());
    
    if i!=usize::MAX && i < m && j < n && j!=usize::MAX && board[i][j] == 'O'{
        board[i][j] = '#';
        
        let opts = [(i -1, j), (i + 1, j), (i, j-1), (i, j+1)];
        
        for (x, y) in opts{
            dfs(board, x, y);
        }
    }
}