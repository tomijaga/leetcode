use std::collections::{BTreeSet};

#[derive(Default)]
struct SmallestInfiniteSet {
    set: BTreeSet<i32>,
    n: i32
}

impl SmallestInfiniteSet {

    fn new() -> Self {
        Self{
            n: 1,
            ..Default::default()
        }
    }
    
    fn pop_smallest(&mut self) -> i32 {
        if self.set.len() == 0{
            let tmp = self.n;
            self.n+=1;
            tmp
        }else{
            let &min = self.set.range(..).next().unwrap();
            
            if min < self.n{
                self.set.remove(&min);
                min
            }else{
                let tmp = self.n;
                self.n+=1;
                tmp
            }
        }
    }
    
    fn add_back(&mut self, num: i32) {
        if num < self.n{
            self.set.insert(num);
        }
    }
}
