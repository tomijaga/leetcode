 impl Solution {
     
     
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        
        
        let mut left = 0;
        let mut right = numbers.len()-1;
        while(numbers[left] + numbers[right] != target ){
            let l = numbers[left];
            let r = numbers[right];
            
            if (l + r < target){
                left+=1;
            }else if (l + r > target){
                right-=1;
            }
        }
        
        return vec![(left +1) as i32, (right + 1)  as i32];
    }
}