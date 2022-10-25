
use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        
        let mut brackets = HashMap::from([
            ('{', '}'), ('(', ')'), ('[', ']')
        ]);
        
        for c in s.chars(){
            if (brackets.contains_key(&c)){
                stack.push(c);
                continue;
            }else if let Some(b) = stack.pop(){
                if let Some(&end) = brackets.get(&b){
                    if end == c{
                        continue;
                    }
                }
            }
            
            return false;
        }
        
        stack.is_empty()
    }
}