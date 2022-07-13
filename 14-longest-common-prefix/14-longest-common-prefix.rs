impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut min = usize::MAX;
        let first = strs[0].clone();
        
        for v in strs.windows(2){
            let mut cnt = 0;
            for (c1, c2) in v[0].chars().zip(v[1].chars()){
                if c1 == c2{
                    cnt+=1;
                }else{
                    break;
                }
            }
            
            if cnt == 0{
                return "".to_owned()
            }else if cnt < min {
                min = cnt
            }
        }
        
        first.chars().take(min).collect()
    }
}

