use std::collections::{VecDeque, HashSet};
use std::iter::FromIterator;

#[derive(Default)]
struct SnakeGame {
    set: HashSet<Vec<i32>>,
    snake : VecDeque<Vec<i32>>,
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
            snake: VecDeque::from([vec![0;2]]),
            set: HashSet::from_iter([vec![0;2]]),
            food
        }
    }
    
    fn make_a_move(&mut self, direction: String) -> i32 {
        let mut head = self.snake.front().unwrap().clone();
        
        match &direction[..]{
            "R" => head[1] += 1,
            "L" => head[1] -= 1,
            "U" => head[0] -= 1,
            "D" => head[0] += 1,
            _ => unreachable!()
        }
        
        self.snake.push_front(head.clone());

        if self.snake.front() != self.food.last(){
            let tail =  self.snake.pop_back().unwrap();
            self.set.remove(&tail);
        }else{
            self.food.pop();
        }

        if head[0] >= 0 && head[0] < self.height && head[1] >= 0 && head[1] < self.width && !self.set.contains(&head) {
            self.set.insert(head);
            (self.snake.len() - 1) as i32
        }else{
            -1
        }
    }
}
