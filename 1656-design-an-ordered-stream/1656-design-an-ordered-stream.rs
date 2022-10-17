struct OrderedStream {
    ptr : usize,
    store : Vec<Option<String>>
}


impl OrderedStream {

    fn new(n: i32) -> Self {
        Self{
            ptr: 0,
            store : vec![None; n as usize]
        }
    }
    
    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        let id_key = (id_key - 1) as usize;
        self.store[id_key] = Some(value);
        let mut res = vec![];
        
        if id_key == self.ptr{
            while let Some(Some(val)) = self.store.get(self.ptr){
                res.push(val.clone());
                self.ptr+=1;
            }
        }
        
        res
    }
}
