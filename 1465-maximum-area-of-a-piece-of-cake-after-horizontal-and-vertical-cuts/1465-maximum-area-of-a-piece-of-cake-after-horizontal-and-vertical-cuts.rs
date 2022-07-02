impl Solution {
    pub fn max_area(h: i32, w: i32, mut horizontal_cuts: Vec<i32>, mut vertical_cuts: Vec<i32>) -> i32 {
        horizontal_cuts.sort_unstable();
        horizontal_cuts.push(h);
        
        vertical_cuts.sort_unstable();
        vertical_cuts.push(w);
        
        let mut prev = 0;
        
        for n in horizontal_cuts.iter_mut(){
            if h < *n{
                break;
            } 
            
            let tmp = prev;
            prev = *n;
            *n -= tmp;
        }
        
        prev = 0;
        
        for n in vertical_cuts.iter_mut(){
            let tmp = prev;
            prev = *n;
            *n -= tmp;
        }
        
        let a = *horizontal_cuts.iter().max().unwrap() as i64;
        let b = *vertical_cuts.iter().max().unwrap() as i64;
        // println!("{:?}\n{:?}", horizontal_cuts, vertical_cuts);
     
        
        ((a * b) % (10_i64.pow(9) + 7)) as i32
    }
}