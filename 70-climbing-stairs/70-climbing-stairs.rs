//  time complexity : O(n)
// space complexity : O(1)

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1{
            return 1;
        }
        
        let (mut one, mut two) = (1, 1);

        for i in 1..n{
            let tmp = one;
            one = two + one;
            two = tmp;
        }
        
        one
    }
}