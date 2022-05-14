use std::collections::HashSet;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut set: HashSet<&str> = HashSet::new();
        let mut duplicates: HashSet<&str> = HashSet::new();
        
        for i in 0..(s.len() as i32 - 9){
            println!("{}", i);
            let i = i as usize;
            let seq = &s[i..(i + 10)];
            
            if let Some(_) = set.get(seq){
                duplicates.insert(seq);
            }else{
                set.insert(seq);
            }
        }
        
        duplicates.into_iter().map(|s|{s.to_string()}).collect::<Vec<String>>()
    }
}