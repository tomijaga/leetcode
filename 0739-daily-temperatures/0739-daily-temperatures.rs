use std::mem;

impl Solution {
    pub fn daily_temperatures(mut temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut res = vec![0; temperatures.len()];
        
        for (j, temp) in temperatures.into_iter().enumerate(){
            
            while let Some((last_temp, i)) = stack.pop(){
                if temp > last_temp{
                    res[i] = (j - i) as i32;
                }else{
                    stack.push((last_temp, i));
                    break;
                }
            }
            
            stack.push((temp, j));
        }
        
        mem::drop(stack);
        
        res
    }
}