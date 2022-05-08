impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut row = vec![1];
        
        
        for _ in 0..row_index{
            let prev_row = row.clone();
            for i in 1..prev_row.len(){
                row[i] = prev_row[i] + prev_row[i - 1];
            }
            
            row.push(1);
        }
        
        row
    }
}