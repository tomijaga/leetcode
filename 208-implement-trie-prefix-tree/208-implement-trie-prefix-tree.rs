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
    
    fn search(&self, word: String) -> bool {
        let mut curr = self;
        
        for c in word.chars().map(|c|{ id(c) }){
            
            if let Some(ref node) = curr.children[c]{
                curr = node.as_ref();
            }else{
                return false;
            }
        }
        
        return curr.is_end;
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut curr = self;
        
        for c in prefix.chars().map(|c|{ id(c) }){
            
            if let Some(ref node) = curr.children[c]{
                curr = node.as_ref();
            }else{
                return false;
            }
        }
        
        return true;
    }
}

pub fn id(c: char) -> usize {
    c as usize - 'a' as usize
}