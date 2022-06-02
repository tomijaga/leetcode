use std::cmp::Ordering;

impl Solution {
    pub fn chalk_replacer(mut chalk: Vec<i32>, k: i32) -> i32 {
        let mut sum:i64 = 0;
        
        let chalk:Vec<i64> = chalk.iter().map(|&n|{
            sum += n as i64;
            sum
        }).collect();
        
        println!("running sum: {:?}", &chalk);
        
        let last = chalk.last().unwrap();
        
        let k = k as i64 % last;
        println!("last: {:?}, k: {:?}", last, k);
        
        
        if k == 0{
            return 0;
        }
        
        let mut l = 0;
        
        let mut r = chalk.len() - 1;
        
        while(l < r){
            let mid = l + (r - l)/2;
            
            let chalk_sum = chalk[mid];
            
            match  get_order(&chalk, mid, k){
                Ordering::Greater => l = mid +1,
                Ordering::Less => r = mid,
                Ordering::Equal => return mid as i32
            }
        }
        r as i32
    }
}

pub fn get_order(chalk:  &Vec<i64>, i: usize, k: i64)-> Ordering{
    println!("{:?}", i);
    if i == 0{
        if chalk[0] - k > 0{
            return Ordering::Equal;
        }else{
            return Ordering::Greater;
        }
        
    } else{
        let n = chalk[i] - chalk[i - 1];
        
        let diff = k - chalk[i - 1];
        
        if diff >= 0 && diff < n{
            return Ordering::Equal;
        }else if diff < 0{
            return Ordering::Less;
        }else{
            return Ordering::Greater;
        }
    }
}