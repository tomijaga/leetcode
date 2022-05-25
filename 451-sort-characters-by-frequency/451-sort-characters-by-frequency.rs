use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut map = HashMap::new();
        
        for c in s.chars(){
            if let Some(frequency) = map.get_mut(&c){
                *frequency+=1;
            }else{
                map.insert(c, 1);
            }
        }
        
        let mut chars = s.chars().collect::<Vec<char>>();
        
        chars.sort_unstable_by(|a, b|{
            let a_n = map.get(a).unwrap();
            let b_n = map.get(b).unwrap();

            if a_n == b_n{
                (b_n + *b as u32).cmp(&(a_n + *a as u32))
            }else{
                (b_n).cmp(&a_n)
            }
        });
        chars.into_iter().collect::<String>()
    }
}