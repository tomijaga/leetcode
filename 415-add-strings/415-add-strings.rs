use std::iter;

impl Solution {
    pub fn add_strings(n1: String, n2: String) -> String {
        let (small, large) = if n1.len() < n2.len(){
             (n1, n2)
        }else{
            (n2, n1)
        };

        let mut carry = 0;

        let mut s:Vec<char> = vec![];
        
        let large_iter = large.chars().rev();
        let small_iter = small.chars().rev();
        let pad_with_zeroes = (0..).map(|_| '0');
        
        let zipped_iter = large_iter.zip(small_iter.chain(pad_with_zeroes));
        
        for (a, b) in zipped_iter {
            let sum = (a as u8 + b as u8) - ('0' as u8 * 2) + carry;
            
            if sum > 9{
                carry = sum / 10;
            }else{
                carry = 0;
            }
            
            s.push((sum % 10 + '0' as u8) as char);
            
        }
        
        if carry > 0{
            s.push((carry % 10 + '0' as u8) as char);
        }

        s.into_iter().rev().collect()
    }
}

