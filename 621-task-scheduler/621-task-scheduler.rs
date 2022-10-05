use std::collections::{BinaryHeap, VecDeque, HashMap};

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut q = VecDeque::new();
        let mut map = HashMap::new();
        
        for c in tasks.into_iter(){
            *map.entry(c).or_insert(0) += 1;
        }
        
        for (c, cnt) in map{
            heap.push((cnt, c));
        }
                
        let mut time = 0;
        
        while !heap.is_empty() || !q.is_empty() {
            // println!("{:?}", (&heap, &q));
            
            while let Some(item) = q.pop_front(){
                let (process_time, cnt, c) = item;
                
                if process_time < time{
                    heap.push((cnt, c));
                }else{
                    q.push_front(item);
                    break;
                }
            }
            
            if let Some((cnt, c)) = heap.pop(){
                if cnt > 1{
                    q.push_back((time + n, cnt - 1, c));
                }
                
                // println!("{}s {:?}", time, c);
            }
            
            time+=1;
        }
        
        time
        
    }
}