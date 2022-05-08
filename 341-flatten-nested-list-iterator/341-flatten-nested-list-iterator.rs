// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }

#[derive(Debug)]
struct NestedIterator {
    array: Vec<i32>,
    i: i32
}

impl NestedInteger{
    pub fn unwrap(&mut self)-> Vec<i32> {
        match self {
            NestedInteger::List(vec) => {
                let mut f = vec![];
                for iter in vec.iter_mut(){
                    f.extend(iter.unwrap());
                    // println!("{:?}", &f);
                }

                f
            },
            NestedInteger::Int(n) => vec![*n]
        }
    }
}

pub fn flatten(nestedList: Vec<NestedInteger>) -> Vec<i32> {
    let mut flat = vec![];
    let mut nestedList = nestedList;
    
    for listEnum in nestedList.iter_mut(){
        flat.extend(listEnum.unwrap());
        // println!("{:?}", &flat);
    }
    
    flat.reverse();
    flat
}


impl NestedIterator {
    fn new(nestedList: Vec<NestedInteger>) -> Self {
        Self{
            array: flatten(nestedList),
            i: 0
        }
    }
    
    fn next(&mut self) -> i32 {
        self.array.pop().unwrap()
    }
    
    fn has_next(&self) -> bool {
        self.array.len() > 0
    }
}
