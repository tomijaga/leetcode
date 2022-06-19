impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, height.len() - 1);
        
        let mut max = 0;
        
        while (l < r){
            let (h1, h2) = (height[l], height[r]);
            let h = h1.min(h2);
            
            let d = (r - l) as i32;
            let area = d * h;
            
            if area > max{
                max = area;
            }
            
            if h2< h1{
                while r > 0 && height[r] <= h2{
                    r-=1;
                }
            }else{
                while l < height.len() && height[l] <= h1{
                    l+=1;
                }
            }
        }
        
        max
    }
}