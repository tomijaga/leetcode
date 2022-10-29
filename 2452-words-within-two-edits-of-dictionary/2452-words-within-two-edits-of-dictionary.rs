
impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        let mut trie = Trie::new();
        
        for word in dictionary{
            trie.insert(word);
        }
        
        let mut res = vec![];
        
        for query in queries{
            if trie.search(&query, 0){
                res.push(query);
            }
        }
        
        res
    }
}

#[derive(Debug, Clone, Default)]
struct Trie{
    is_end: bool,
    children: [Option<Box<Trie>>; 26]
}

impl Trie {

    fn new() -> Self {
        Default::default()
    }
    
    fn insert(&mut self, word: String) {
        let mut curr = self;
        
        for c in word.chars().map(|c|{ id(c) }){
            let mut t = Trie::new();
            
            curr = curr.children[c].get_or_insert(Box::new(t)).as_mut();
        }
        
        curr.is_end = true;
    }
    
    fn search(&self, word: &str, changes: i32) -> bool {
        if word.len() == 0{
            return true;
        }
        
        let mut res = false;
        
        let c =  id(word.chars().next().unwrap());
        
        for j in (0..26){
            if let Some(ref node) = self.children[j]{
                if c == j{
                    res |= node.as_ref().search(&word[1..], changes)
                }else if changes < 2{
                    res |= node.as_ref().search(&word[1..], changes + 1)
                };
            }
        } 
        
        res
    }
}

pub fn id(c: char) -> usize {
    (c as u8 - 'a' as u8) as usize 
}
