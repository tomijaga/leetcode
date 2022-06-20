impl Solution {
    pub fn minimum_length_encoding(mut words: Vec<String>) -> i32 {
        let mut res:Vec<String> = vec![];
        
        words.sort_unstable_by(|a, b|{
            b.len().cmp(&a.len())
        });
        
        for newWord in words{
            let is_suffix = !res.iter().any(|word|{
                let (a, b) = (newWord.len(), word.len());
                
                if a<=b{
                    &word[b-a..] == &newWord
                }else{
                    false
                }
            });
            
            if is_suffix {
                res.push(newWord);
            }
        }
        
        res.into_iter().fold(0_i32, |acc, word|{ 
            acc + (word.len() as i32) + 1 
        })
    }
}