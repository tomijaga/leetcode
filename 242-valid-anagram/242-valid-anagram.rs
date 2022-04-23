impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if (t.len() != s.len()){
            return false;
        }
        let mut s_arr = s.chars().collect::<Vec<char>>();
        s_arr.sort_unstable();
        
        let mut t_arr = t.chars().collect::<Vec<char>>();
        t_arr.sort_unstable();
        
        return s_arr == t_arr
    }
}