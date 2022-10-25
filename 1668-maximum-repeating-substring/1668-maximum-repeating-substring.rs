use std::collections::HashMap;

impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        if word.len() > sequence.len() {
            return 0;
        }
        
        let mut map = HashMap::new();
        
        let word_len = word.len();
        let limit = sequence.len() - word.len();
        
        let mut cnt = 0;
        let mut max_len = 0;
        let mut i = 0;
        
        for i in 0..=limit{
            if sequence[i..(i + word_len)] == word[..]{
                map.entry(i % word_len).or_insert(vec![]).push(i / word_len);
            }
        }
        
        for v in map.into_values(){
            // println!("{:?}", &v);
            cnt = 1;
            for w in v.windows(2){
                let a = w[0];
                let b = w[1];

                if a + 1 == b{
                    cnt += 1;
                }else{
                    cnt = 1;
                }

                max_len = max_len.max(cnt);
            }
            max_len = max_len.max(cnt);
        }

        max_len
    }
}