impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (m, n) = (board.len(), board[0].len());
        
        // mark_edges
        for i in 0..m{
            for j in [0, n-1]{
                if board[i][j] == 'O'{
                    dfs(board, i, j);
                }
            }
        }
        
        for j in 0..n{
            for i in [0, m-1]{
                if board[i][j] == 'O'{
                    dfs(board, i, j);
                }
            }
        }
        
        for i in 0..m{
            for j in 0..n{
                if board[i][j] == 'O'{
                    board[i][j] = 'X';
                }else if board[i][j] == '-'{
                    board[i][j] = 'O';
                }
            }
        }
        
    }
}

fn dfs(board: &mut Vec<Vec<char>>, i: usize, j: usize){
    let (m, n) = (board.len(), board[0].len());
    
    board[i][j] = '-';
    
    for (i, j) in [(i+ 1, j), (i-1, j), (i, j+1), (i, j-1)]{
        if i!=usize::MAX && i < m && j!=usize::MAX && j < n && board[i][j] == 'O'{
            dfs(board, i, j);
        }
    }
}

