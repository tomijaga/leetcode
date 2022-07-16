#[derive(Default, Debug, Clone)]
struct Trie {
    is_word_end: bool,
    children: [Option<Box<Trie>>; 26],
}

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut head = Trie::new();
        let (m, n) = (board.len(), board[0].len());
        
        for i in 0..m{
            for j in 0..n{
                Self::dfs_insert(&mut board, &mut head, i, j, 0);
            }
        }
        
        let mut res = vec![];
        
        for word in words{
            if head.search(&word){
                res.push(word);
            }
        }
        
        res
    }
    
    pub fn dfs_insert(board: &mut Vec<Vec<char>>, mut trie: &mut Trie, i: usize, j: usize, len: u8){
        let (m, n) = (board.len(), board[0].len());
        
        if i != usize::MAX && j!= usize::MAX && i < m && j < n && board[i][j] != '#' && len < 10{
            let c = board[i][j];
            board[i][j] = '#';
            
            trie = trie.children[char_index(c)]
                .get_or_insert(Box::new(Trie::new()))
                .as_mut();
            
            for (x, y) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)]{
                Self::dfs_insert(board, trie, x, y, len+1);
            }
            
            board[i][j] = c;
        }
    }
}


impl Trie {

    fn new() -> Self {
        Default::default()
    }
    
    fn insert(&mut self, word: &str) {
        let mut trie = self;
        
        for c in word.chars(){
            trie = trie.children[char_index(c)]
                .get_or_insert(Box::new(Trie::new()))
                .as_mut();
        }
        
        trie.is_word_end = true;
    }
    
    fn search(&self, word: &str) -> bool {
        let mut trie = self;
        
        for c in word.chars(){
            if let Some(ref next_trie) = trie.children[char_index(c)]{
                trie = next_trie.as_ref();
            }else{
                return false;
            }
        }
        
        true
    }
}

pub fn char_index(c: char) -> usize{
    c as usize - 'a' as usize
}