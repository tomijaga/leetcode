use std::collections::HashMap;
use std::cmp::Ordering;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        
        for n in nums{
            *map.entry(n).or_default() +=1;
        }
        
        let mut arr: Vec<(i32, i32)> = map.into_iter().collect();

        let res = if k == arr.len() as i32{
            &arr
        }else{
            quick_select(&mut arr, k)
        };
        
        res.into_iter()
        .map(|&(n, _)| n)
        .collect()
    }
}

pub fn quick_select(slice: &mut [(i32, i32)], k: i32) -> &[(i32, i32)]{
    let (mut pivot, mut i, mut j) = (0, 1, 1);
    
    for index in 1..slice.len(){
        if slice[index].1 >= slice[pivot].1{
            swap(slice, index, j);
            j+=1;
        }else{
            swap(slice, index, i);
            i+=1;
            j+=1;
        }
    }
    
    swap(slice, pivot, i - 1);
    
    pivot = i - 1;
    i-=1;
    
    let greater_items = (j - pivot) as i32;
    // println!("{:?} {:?}", &slice[i..j], greater_items);
    
    match greater_items.cmp(&k) {
        Ordering::Less => {
            quick_select(&mut slice[0..j], k)
        },
        Ordering::Greater => {
            quick_select(&mut slice[pivot + 1..j], k)
        },
        Ordering::Equal => &slice[pivot..j],
    }
}

pub fn swap(slice: &mut [(i32, i32)], i: usize, j: usize){
    let tmp = slice[i];
    slice[i] = slice[j];
    slice[j] = tmp;
}