impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let arr= ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut stack =  vec![];
        
        for c in digits.chars(){
            let n = c.to_digit(10).unwrap() as usize;
            let chars = arr[n - 2];
            
            let mut new_stack = vec![];
            for c in chars.chars(){

                if stack.len() > 0{
                    for combo in stack.iter(){
                        let mut s = String::from(combo);
                        s.push(c);
                        new_stack.push(s);
                    }
                }else{
                    new_stack.push(c.to_string());
                }
            }
            stack = new_stack;
        } 
        
        stack
    }
}