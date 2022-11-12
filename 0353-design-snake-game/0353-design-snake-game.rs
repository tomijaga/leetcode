use std::collections::{VecDeque, HashSet};
use std::iter::FromIterator;

#[derive(Default)]
struct SnakeGame {
    set: HashSet<(i32, i32)>,
    snake : VecDeque<(i32, i32)>,
    food : Vec<Vec<i32>>,
    height: i32,
    width: i32,
}


impl SnakeGame {

    fn new(width: i32, height: i32, mut food: Vec<Vec<i32>>) -> Self {
        food.reverse();
        
        Self{
            height,
            width,
            snake: VecDeque::from([(0, 0)]),
            set: HashSet::from_iter([(0, 0)]),
            food
        }
    }
    
    fn make_a_move(&mut self, direction: String) -> i32 {
        let mut head = *self.snake.front().unwrap();
        
        match &direction[..]{
            "R" => head.1 += 1,
            "L" => head.1 -= 1,
            "U" => head.0 -= 1,
            "D" => head.0 += 1,
            _ => unreachable!()
        }
        
        self.snake.push_front(head);

        let mut ate_food = false;
        
        if let Some(food) = self.food.last(){
            if head.0 == food[0] && head.1 == food[1]{
                ate_food = true;
            }
        }
        
        if ate_food{
            self.food.pop();
        }else{
            let tail =  self.snake.pop_back().unwrap();
            self.set.remove(&tail);
        }

        if head.0 >= 0 && head.0 < self.height && head.1 >= 0 && head.1 < self.width && !self.set.contains(&head) {
            self.set.insert(head);
            (self.snake.len() - 1) as i32
        }else{
            -1
        }
    }
}