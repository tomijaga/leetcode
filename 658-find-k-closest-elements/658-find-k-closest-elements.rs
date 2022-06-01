impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = arr.len() - 1;
        
        while (left < right){
            let mid = left + (right - left)/2;
            
            let n = arr[mid];
            if n > x{
                right = mid;
                
            }else if n < x{
                left = mid + 1;
                
            }else{
                right = mid;
                break;
            }
        }

        let (mut index_a, mut index_b) = (right as i32 -1, right as i32);
        let mut result = vec![];
        
        for i in 0..k{ 
            if index_a < 0{
                result.push(arr[index_b as usize]);
                index_b+=1;
            }else if index_b as usize >= arr.len(){
                result.push(arr[index_a as usize]);
                index_a-=1;
                
            }else {
                let a = arr[index_a as usize];
                let b = arr[index_b as usize];
                
                if a_is_closer(a, b, x){
                    
                    result.push(a);
                    index_a-=1;
                }else{
                    result.push(b);
                    index_b+=1;
                }
            }
        }
        
        result.sort();
        result
    }
}

pub fn a_is_closer(a: i32, b: i32, c: i32)-> bool{
    ( a - c ).abs() <= ( b - c ).abs()
}