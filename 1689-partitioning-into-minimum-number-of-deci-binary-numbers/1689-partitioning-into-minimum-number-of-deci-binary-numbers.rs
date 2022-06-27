impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut m = '0';
        
        for c in n.chars(){
            if c > m{
                m = c;
            }
        }
        
        m.to_string().parse::<i32>().unwrap()
    }
}