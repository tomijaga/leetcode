struct Vector2D {
    vec: Vec<Vec<i32>>,
    curr: Vec<i32>
}

impl Vector2D {

    fn new(mut vec: Vec<Vec<i32>>) -> Self {
        for v in vec.iter_mut(){
            v.reverse();
        }
        vec.reverse();
        
        let curr = if vec.len() > 0{
            
            let tmp = vec.pop().unwrap();
            
            let mut popped = true;
            while popped{
                popped = false;
                if let Some(last) =  vec.last(){
                    if last.len() == 0{
                        vec.pop();
                        popped = true;
                    }
                }
            }
            tmp
        }else{
            vec![]
        };
        
        Self {vec, curr}
    }
    
    fn next(&mut self) -> i32 {
        if !self.curr.is_empty(){
            self.curr.pop().unwrap()
        }else{
            if !self.vec.is_empty(){
                self.curr = self.vec.pop().unwrap();
                let mut popped = true;
                while popped{
                    popped = false;
                    if let Some(last) =  self.vec.last(){
                        if last.len() == 0{
                            self.vec.pop();
                            popped = true;
                        }
                    }
                }
                self.curr.pop().unwrap()
            }else{
                -1
            }
        }
    }
    
    fn has_next(&self) -> bool {
        // println!("{:?}", (&self.curr, &self.vec) );
        !self.curr.is_empty() || !self.vec.is_empty()
    }
}