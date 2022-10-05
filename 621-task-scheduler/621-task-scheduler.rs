use std::collections::{BinaryHeap, VecDeque};

impl Solution {
    pub fn least_interval(mut tasks: Vec<char>, n: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut q = VecDeque::new();
        let mut map = vec![0; 26];
        
        let mut cnt = 0;
        let mut prev = tasks[0];
        
        tasks.sort_unstable();
        
        for t in tasks{
            map[(t as u8 - 'A' as u8) as usize] +=1;
        }
        
        for (i, cnt) in map.into_iter().enumerate(){
            if cnt > 0{
                let c = ('A' as u8 + i as u8) as char;
                heap.push((cnt, c));
            }
        }
        
        let mut time = 0;
        
        while !heap.is_empty() || !q.is_empty() {
            
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
            }
            
            time+=1;
        }
        
        time
        
    }
}