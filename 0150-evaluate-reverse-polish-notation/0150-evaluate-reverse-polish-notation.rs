impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        
        for s in tokens{
            match &s[..] {
               "*" | "+" | "-" | "/" => {
                    let b = stack.pop().unwrap();
                    let a = stack.last_mut().unwrap();
                    operation(a, b, &s)
                },
                _ => stack.push(s.parse().unwrap()),
            }
        }
        
        stack.pop().unwrap()
    }
}

fn operation(a: &mut i32, b: i32, op: &str){
    *a = match(op){
        "+" => *a + b,
        "-" => *a - b,
        "*" => *a * b,
        "/" => *a / b,
        _  => unreachable!()
    }
}