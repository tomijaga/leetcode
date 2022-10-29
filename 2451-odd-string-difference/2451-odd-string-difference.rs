use std::collections::HashMap;
impl Solution {
    pub fn odd_string(words: Vec<String>) -> String {
        let mut map:HashMap<Vec<i8>, Vec<String>> = HashMap::new();
        
        for word in words{
            let mut words_iter = word.chars();
            
            let mut prev = words_iter.next().unwrap();
            let mut diff = vec![];
            
            for curr in words_iter{
                diff.push(pos(curr) - pos(prev));
                prev = curr;
            }
            
            if let Some(v) = map.get_mut(&diff){
                if v.len() < 2{
                    v.push(word);
                }
            }else{
                map.insert(diff, vec![word]);
            }
        }
        
        for (_, v) in map{
            if v.len() == 1{
                return v[0].clone();
            }
        }
        
        unreachable!()
    }
}

fn pos(c: char) -> i8{
    c as i8 - 'a' as i8 
}