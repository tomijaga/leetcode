impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = vec![];
        let mut s = String::new();
        
        fn backtrack(res: &mut Vec<String>, s: &mut String, n:i32,  open: i32, close: i32){
            if open == n && close == n{
                res.push(s.clone());
                return;
            }
            
            if open < n{
                s.push('(');
                backtrack(res, s, n, open+1, close);
                s.pop();
            }
            
            if open > close{
                s.push(')');
                backtrack(res, s, n, open, close+1);
                s.pop();
            }
        }
        
        backtrack(&mut res, &mut s, n, 0, 0);
        
        res
    }
}