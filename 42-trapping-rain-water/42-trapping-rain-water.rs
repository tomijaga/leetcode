impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        
        let mut l = vec![height[0]];
        let mut r = vec![*height.last().unwrap()];
        
        for &h in height[1..].iter(){
            l.push(h.max(*l.last().unwrap()));
        }
        
        for &h in (height[..height.len() - 1].iter()).rev(){
            r.push(h.max(*r.last().unwrap()));
        }
        
        r.reverse();
        
        for (i, (h1, h2)) in l.iter_mut().zip(r.into_iter()).enumerate(){
            *h1 = (*h1).min(h2);
        }
        
        let mut sum = 0;
        
        for (h, minLR) in height.into_iter().zip(l.into_iter()){
            if h< minLR{
                sum += minLR - h;
            }
        }
        
        sum
    }
}