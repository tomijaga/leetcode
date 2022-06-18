use std::collections::HashMap;

#[derive(Debug)]
struct Node(HashMap<char, Node>, bool);

#[derive(Debug)]
struct Trie {
    data: HashMap<char, Node>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Self{
            data: HashMap::new()
        }
    }
    
    fn insert(&mut self, word: String) {
        let mut data = &mut self.data;
        let len = word.len();
        let w = word.clone();
        
        for (i, c) in word.chars().enumerate(){
            
            if !data.contains_key(&c){
                data.insert(c, Node(HashMap::new(), i == len - 1));
            }else{
                if i == len - 1{
                    data.get_mut(&c).unwrap().1 = true;
                }
                // println!("{:?}", (&w, c));
            }
            data = &mut data.get_mut(&c).unwrap().0;
        }
        
        // println!("{:#?}", &self.data);
    }
    
    fn search(&self, word: String) -> bool {
        let mut data = (&self.data, false);
        
        for c in word.chars(){
            if !data.0.contains_key(&c){
                return false;
            }
            let ref_data = &data.0.get(&c).unwrap();
            data = (&ref_data.0, ref_data.1);
        }
        
        return data.1;
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut data = &self.data;
        
        for c in prefix.chars(){
            if !data.contains_key(&c){
                return false;
            }
            let ref_data = &data.get(&c).unwrap();
            data = &ref_data.0;
        }
        
        return true;
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */