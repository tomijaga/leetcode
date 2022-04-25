pub fn isOdd(n: i32)-> bool{
    n %2 == 1
}

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut odds = 0;
        if arr.len() < 3{
            return false;
        }
        
        for i in 0..arr.len() - 2{
            if isOdd(arr[i]) && isOdd(arr[i+1]) && isOdd(arr[i+2]){
                return true;
            }
        }
        
        false
    }
}