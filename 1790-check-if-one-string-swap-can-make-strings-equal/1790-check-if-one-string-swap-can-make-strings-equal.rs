impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut map = vec![0; 26];
        
        let len = s1.len();
        let mut matches = 0;
        
        for (a, b) in s1.chars().zip(s2.chars()){
            map[id(a)] +=1;
            map[id(b)] -=1;
            
            matches += if a == b { 1 } else { 0 };
        }
        
        let diff = len - matches;

        map.into_iter().all(|cnt| cnt == 0) && (diff == 0 || diff == 2)
    }
}

fn id(c: char) -> usize {
    (c as u8 - 'a' as u8) as usize
}