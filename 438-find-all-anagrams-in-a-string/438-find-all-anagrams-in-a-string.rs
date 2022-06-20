impl Solution {
    pub fn find_anagrams(s: String, mut p: String) -> Vec<i32> {
        if p.len() > s.len() {
            return vec![];
        }
        
        let mut p : Vec<char> = p.chars().collect();
        p.sort_unstable();
        
        let mut res = vec![];
        
        for i in 0..=(s.len() - p.len()){
            let mut slice = (&s[i..(i + p.len())]).chars().collect::<Vec<char>>();
            slice.sort_unstable();
            
            if slice == p{
                // println!("{:?}", (slice, &p));
                res.push(i as i32);
            }
        }
        
        res
    }
}