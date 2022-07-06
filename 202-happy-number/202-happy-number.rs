use std::collections::HashSet;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut set = HashSet::new();
        
        while !set.contains(&n){
            set.insert(n);
            n = split_digits_sum(n);
            
            // println!("n: {:?}", n);
            if n == 1{
                return true;
            }
        }
        
        false
    }
}

pub fn split_digits_sum(num: i32)-> i32{
    let mut num = num;
    let mut sum = 0_i32;
    while num > 0{
        sum += (num % 10).pow(2);
        num/=10;
    }
    
    sum
}