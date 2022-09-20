use std::collections::BTreeSet;

#[derive(Default)]
struct TicTacToe {
    x: BTreeSet<(i32, i32)>,
    o: BTreeSet<(i32, i32)>,
    size: i32
}

impl TicTacToe {

    fn new(n: i32) -> Self {
        Self{
            size: n,
            ..Default::default()
        }
    }
    
    fn make_a_move(&mut self, row: i32, col: i32, player: i32) -> i32 {
        let set = if player == 1{
            &mut self.x
        }else{
            &mut self.o
        };
        
        set.insert((row, col));
        
        let mut pos = vec![0;4];
        
        for i in 0..self.size{
            if set.contains(&(row, i)){
                pos[0]+=1;
            }
            
            if set.contains(&(i, col)){
                pos[1]+=1;
            }
            
            if set.contains(&(i, i)){
                pos[2]+=1;
            }
            
            if set.contains(&(i, self.size - i - 1)){
                pos[3]+=1;
            }
        }
        
        if pos.into_iter().any(|val| val == self.size){
            player
        }else{
            0
        }
    }
}