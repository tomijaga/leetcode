impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if (t.len() != s.len()){
            return false;
        }
        let mut s_arr = s.chars().collect::<Vec<char>>();
        s_arr.sort();
        
        let mut t_arr = t.chars().collect::<Vec<char>>();
        t_arr.sort();
        
        let s: String = s_arr.iter().collect();
        
        for (i, c) in s.chars().enumerate(){
            if (c != t_arr[i]){
                return false;
            }
        }
        true
    }
}