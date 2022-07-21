use std::iter;

impl Solution {
    pub fn add_strings(n1: String, n2: String) -> String {
        let (n1, n2) = if n1.len() < n2.len(){
             (n1, n2)
        }else{
            (n2, n1)
        };

        let mut carry = 0;

        let mut s:Vec<char> = vec![];
        for (a, b) in n2.chars()
                        .rev()
                        .zip(n1.chars()
                                .rev()
                                .chain((0..).map(|_| '0')))
        {
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

