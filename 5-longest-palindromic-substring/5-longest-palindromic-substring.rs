impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() <=1{
            return s;
        }
        
        let chars: Vec<char> = s.chars().collect();
        let mut max = (0, 1);
        
        for i in 0..s.len() -1{
            longest(&chars, i, i, &mut max);
            longest(&chars, i, i + 1, &mut max);
        }
        s[max.0..(max.0 + max.1)].to_string()
    }
}

pub fn longest(s: &Vec<char>, i: usize, j: usize, max: &mut(usize, usize)){
    let (mut i, mut j) = (i, j);
    let mut start = i;
    
    let mut len = 0;
    while i >=0 && i!=usize::MAX && j<s.len() && s[i] == s[j]{
        // println!("{},{:?}, {:?}",start, (i,j), &s[i..=j]);
        len = j -i +1;
        start = i;
        i-=1;
        j+=1;
    }
    
    
    if len > max.1{
        max.0 = start;
        max.1 = len;
    }
    
}