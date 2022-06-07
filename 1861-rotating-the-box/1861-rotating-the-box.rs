impl Solution {
    pub fn rotate_the_box(b: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let (m, n) = (b.len(), b[0].len());
        
        let mut flipped_box = vec![vec!['.'; m]; n];
        
        for i in 0..m{
            let mut pos = n-1;
            
            for j in (0..n).rev(){
                match (b[i][j]) {
                    '*' => {
                        flipped_box[j][m - i - 1] = '*';
                        pos = j - 1;
                    },
                    '#' => {
                        flipped_box[pos][m - i - 1] = '#';
                        pos -=1;
                    },
                    _ => {}
                }
            }
        }
        
        flipped_box
    }
}