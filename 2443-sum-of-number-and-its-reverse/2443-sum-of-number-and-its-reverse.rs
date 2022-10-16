impl Solution {
    pub fn sum_of_number_and_reverse(num: i32) -> bool {
        if num == 0 { return true };
        
        for n in (num/2..num).rev(){
            if n + reverse(n) == num{
                return true;
            }
        }
        
        false
    }
}

fn reverse(mut n: i32) -> i32 {
    let mut reversed = 0;
    
    while n > 0{
        reversed = (n % 10) + (reversed * 10);
        n/= 10;
    }
    
    reversed
}