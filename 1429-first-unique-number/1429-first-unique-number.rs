// extra space : O(2*k) -> O(k)
// where k is the number of unique numbers 

// methods time complexity:
//   new() : O(n log k)
//   add() : O(log k)
//   show_first_unique() : O(k), Θ(1)
//      The highest runtime of this algorithms is O(k) because
//      the heap can remove up to k elements if none of them are unique.
//      When this function is called after this process of eliminating 
//      duplicates, the algorithm will run in O(1) time 
//
//      This is similar to the push function for a dynamic array
//      If called when the internal array is full it will have to 
//      create an new array of size 2n and copy all the values from
//      old array to the new one. O(n)
//      Normally the run time is O(1)


use std::collections::{HashMap, BinaryHeap};

struct FirstUnique {
    map: HashMap<i32, i32>,
    heap: BinaryHeap<(i32, i32)>,
    next: i32
}

impl FirstUnique {

    fn new(nums: Vec<i32>) -> Self {
        let mut map = HashMap::new();
        let mut heap = BinaryHeap::new();
        let mut next = 0;
        
        for (i, n) in nums.into_iter().enumerate(){
            let i = i as i32;
            if !map.contains_key(&n){
                heap.push((-next, n));
                next +=1;
            }
            *map.entry(n).or_default() +=1;
        }
        
        Self{ map, heap, next }
    }
    
    fn show_first_unique(&mut self) -> i32 {
        let mut peekedItem = &(0,0);
        if self.heap.len() == 0{
           return -1;
        }else{
            peekedItem = self.heap.peek().unwrap();
        }
        
        while let Some(&cnt) = self.map.get(&peekedItem.1){
            if cnt > 1{
                self.heap.pop();
                if self.heap.len() == 0{
                    return -1;
                }else{
                    peekedItem = self.heap.peek().unwrap();
                }
            }else{
                break;
            }
        }
        
        peekedItem.1
    }
    
    fn add(&mut self, value: i32) {
        if !self.map.contains_key(&value){
            self.heap.push((-self.next, value));
        }
        
        *self.map.entry(value).or_default() +=1;
        
        self.next+=1;
    }
}