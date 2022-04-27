use std::collections::HashMap;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut players = HashMap::new();
        
        for tuple in matches{
            let winner = tuple[0];
            let loser = tuple[1];
            
            if (!players.contains_key(&winner)){
                players.insert(winner, 0);
            }
            
            if let Some(losses) = players.get_mut(&loser){
                *losses +=1;
            }else{
                players.insert(loser, 1);
            }
        }
        
        // println!("{:?}", players);
        
        let mut no_losses = vec![];
        let mut one_loss = vec![];
        
        for (&player, &losses) in players.iter(){
            if (losses == 0){
                no_losses.push(player);
            } else if (losses ==1){
                one_loss.push(player);
            }
        }
        
        no_losses.sort();
        one_loss.sort();
        
        vec![no_losses, one_loss]
    }
}