use rand::prelude::*;

struct Solution {
    rand: ThreadRng, 
    v: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(mut w: Vec<i32>) -> Self {
        let mut rand = thread_rng();
        let mut sum = 0;
        for n in w.iter_mut(){
            sum +=*n;
            *n = sum;
        }
        
        // println!("{:?}", &w);
        
        Self{
            rand,
            v: w
        }
    }
    
    fn pick_index(&mut self) -> i32 {
        let rnd = self.rand.gen_range(1, *self.v.last().unwrap() + 1);
        let rnd = rnd as i32;
        // println!("{:?}", &rnd);
        
        let mut l = 0;
        let mut r = self.v.len() - 1;
        
        while l < r {
            let mid = l + (r - l)/2;
            let n = self.v[mid];
            
            // println!("- {:?}, {}", (l, r), n);
            
            if rnd < n{
                r = mid;
            }else if rnd > n{
                l = mid + 1;
            }else{
                return mid as i32;
            }
        }
        
        r as i32
    }
}