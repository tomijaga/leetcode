impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        
        fn edit(acc: &mut Vec<char>, c: char) -> &mut Vec<char>{
            if c == '#'{
                acc.pop();
            }else{
                acc.push(c);
            }
            
            acc
        };
        
        let mut a = vec![];
        let mut b = vec![];
        
        s.chars().fold(&mut a, edit);
        t.chars().fold(&mut b, edit);
        
        
        a == b
    }
}