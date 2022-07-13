impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = vec![""];
        
        for p in path.split("/"){
            if p == ""{
                continue;
            }
            
            if p == ".."{
                if stack.len() > 1{
                    stack.pop();
                }
            }else if p != "."{
                stack.push(p);
            }
        }
        
        if stack.len() == 1{
            String::from("/")
        }else{
            stack.join("/")
        }
    }
}