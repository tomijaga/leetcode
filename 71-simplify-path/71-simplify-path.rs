impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = vec![];
        
        for p in path.split("/"){
            if p == ""{
                continue;
            }
            
            if p == ".."{
                if stack.len() > 0{
                    stack.pop();
                }
            }else if p != "."{
                stack.push(p);
            }
        }
        
        let mut cwd = String::from("/");
        cwd.push_str(&stack.join("/"));
        cwd
    }
}