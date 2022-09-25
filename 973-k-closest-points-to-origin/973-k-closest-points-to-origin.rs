use std::collections::BinaryHeap;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut heap = BinaryHeap::new();
        
        for point in points{
            let (x, y) = (point[0], point[1]);
            let dx = (x - 0).pow(2);
            let dy = (y - 0).pow(2);
            
            heap.push((dx + dy, vec![x, y]));
            
            if heap.len() > k{
                heap.pop();
            }
        }
        
        heap.into_iter().map(|(d, v)| v).collect()
    }
}