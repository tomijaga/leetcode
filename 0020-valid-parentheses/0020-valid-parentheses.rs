
use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        
        let mut brackets = HashMap::from([('{', '}'), ('(', ')'), ('[', ']')]);
        
        for c in s.chars(){
            if (brackets.contains_key(&c)){
                stack.push(c);
            }else if let Some(bracket) = stack.pop(){
                if let Some(&end_bracket) = brackets.get(&bracket){
                    if c != end_bracket{
                        return false;
                    }
                }
            }else{
                return false;
            }
        }
        
        stack.is_empty()
    }
}