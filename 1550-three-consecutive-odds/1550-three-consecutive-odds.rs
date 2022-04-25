impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut odds = 0;
        
        for n in arr{
            if n %2 == 1{
                odds+=1;
            }else{
                odds = 0;
            }
            
            if odds ==3{
                return true;
            }
        }
        
        false
    }
}