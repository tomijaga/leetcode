pub fn calc_steps(step: (i32, i32))-> (i32,i32){
    let (num, count) = step;
    
    if num == 0{
        step
    }else if num % 2 == 0{
        calc_steps((num/2, count +1))
    }else{
        calc_steps((num - 1 , count +1))
    }
}

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        calc_steps((num, 0)).1
    }
}