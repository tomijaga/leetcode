impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        
        fn cbns(res: &mut Vec<Vec<i32>>, candidates: &[i32], tuple: (Vec<i32>, i32), target: i32 ){
            if candidates.len() == 0 {
                return;
            }
            
            let n = candidates[0];
            let (nums, sum) = tuple;
            
            for (i, &n) in candidates.iter().enumerate(){
                let sum = sum + n;
                if sum <= target{
                    let mut new_nums = nums.clone();
                    new_nums.push(n);
                    // println!("new nums: {:?}, sum: {:?}", &new_nums, sum);
                    
                    if sum == target{
                        res.push(new_nums);
                    }else{
                        if i < candidates.len(){
                            cbns(res, &candidates[i..], (new_nums, sum), target);
                        }
                    }
                }
            }
        }
        
        cbns(&mut res, &candidates, (vec![], 0), target);
        
        res
    }
}