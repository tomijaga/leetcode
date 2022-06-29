use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut map = HashMap::from([
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z'])
        ]);
        
        let mut res = vec![];
        let digits: Vec<char> = digits.chars().collect();
        
        cbns(&digits, &map, &mut res, String::new());
        
        res
    }
}

pub fn cbns(digits: &[char], map: &HashMap<char, Vec<char>>, res: &mut Vec<String>, nums: String){
    if digits.len() == 0{
        if nums.len() > 0{
            res.push(nums);
        }
        return;
    }
    let n = digits[0];
    
    for &c in map.get(&n).unwrap(){
        let mut new_nums = nums.clone();
        new_nums.push(c);
        
        cbns(&digits[1..], map, res, new_nums);
    }
}