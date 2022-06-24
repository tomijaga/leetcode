use std::collections::BinaryHeap;

impl Solution {
    pub fn is_possible(mut arr: Vec<i32>) -> bool {
        let target = vec![1; arr.len()];
        
        let mut sum: i32 = arr.iter().sum();
        let mut heap = BinaryHeap::from(arr);
        
        
        loop{
            println!("{:?}", &heap);
            
            let mut n = heap.pop().unwrap();
            let new_sum = sum - n;
            
            
            if n  > new_sum && new_sum != 0{
                let new_n = n % new_sum;
                
                if new_n == 0 {
                    let _n = n - new_sum;
                    if _n == 0{
                        heap.push(n);
                        break;
                    }
                    heap.push(_n);
                    
                }else{
                    heap.push(new_n);
                }

                sum = new_sum + new_n;
                if sum <= 0{
                    break;
                }
            }else{
                heap.push(n);
                break;
            }
        }
        
        let arr = heap.into_iter().collect::<Vec<i32>>();
        // println!("{:?}", &arr);
        target == arr
    }
}