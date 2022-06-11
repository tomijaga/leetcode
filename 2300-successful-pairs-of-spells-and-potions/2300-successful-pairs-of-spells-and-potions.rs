impl Solution {
    pub fn successful_pairs(mut spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort_unstable();
        
        for i in 0..spells.len(){
            let spell = spells[i] as i64;
            
            let mut l = 0;
            let mut r = potions.len();
            
            while (l < r){
                let mid = l + (r - l)/2;
                let potion = potions[mid] as i64;
                
                if spell * potion <  success{
                    l = mid + 1;
                }else{
                    r = mid;
                }
            }
            
            spells[i] = (potions.len() - r ) as i32;
        }
        
        spells
    }
}