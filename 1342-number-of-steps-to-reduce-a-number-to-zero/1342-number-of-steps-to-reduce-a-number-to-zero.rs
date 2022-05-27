pub fn calc_steps(step: (i32, i32))-> (i32,i32){
    if step.0 == 0{
        step
    }else if step.0 % 2 == 0{
        calc_steps((step.0/2, step.1 +1))
    }else{
        calc_steps((step.0 - 1 , step.1 +1))
    }
}

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        calc_steps((num, 0)).1
    }
}