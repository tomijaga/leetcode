impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut stack = vec![0];
        let mut count = 0;
        
        for next in nums {
            let mut new_stack = vec![];
            
            for curr in stack {
                let n1 = curr - next;
                let n2 = curr + next;

                new_stack.push(n1);
                new_stack.push(n2);
            }

            stack = new_stack;
        }
        
        for n in stack{
            if n == target {
                count +=1;
            }
        }

        count
    }
}