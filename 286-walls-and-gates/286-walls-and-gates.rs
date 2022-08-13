use std::collections::VecDeque;

const WALL: i32 = -1;
const GATE: i32 = 0;
const EMPTY: i32 = i32::MAX;

impl Solution {
    pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
        let (m, n) = (rooms.len(), rooms[0].len());
        let mut q = VecDeque::new();
        
        for i in 0..m{
            for j in 0..n{
                if rooms[i][j] == GATE{
                    q.push_back((i, j));
                }
            }
        }
        
        let mut cnt = 0;
        
        while !q.is_empty(){
            cnt +=1;
            println!("{:?}", (cnt, &q));
            
            for _ in 0..q.len(){
                let (i, j) = q.pop_front().unwrap();

                for (i, j) in [(i+1, j), (i-1, j), (i, j+1), (i, j-1)]{
                    if i!=usize::MAX && i < m && j!=usize::MAX && j < n && rooms[i][j] == EMPTY{
                        if rooms[i][j] > cnt{
                            rooms[i][j] = cnt;
                            q.push_back((i, j));
                        }
                    }
                }
            }
        }
    }
}