pub fn fill_mut(image: &mut Vec<Vec<i32>>, sr: usize, sc: usize, prev_color: i32, new_color: i32){
    let (r, c) = (image.len(), image[0].len());
    
    if sr >= r || sc >= c || sc < 0 || sr < 0{
        return;
    }
    
    let val = image[sr][sc];
    // println!("{:?} {:?}", sr, sc);
    if val == prev_color{
        image[sr][sc] = new_color; 
        
        fill_mut(image, sr + 1, sc, prev_color, new_color);
        fill_mut(image, sr - 1, sc, prev_color, new_color);
        fill_mut(image, sr, sc + 1, prev_color, new_color);
        fill_mut(image, sr, sc - 1, prev_color, new_color);
    }
    
}

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let mut image = image;
        let sr = sr as usize;
        let sc = sc as usize;
        
        let color = image[sr][sc];
        if color != new_color{
           fill_mut(&mut image,sr, sc, color, new_color); 
        }
        
        image
    }
}