impl Solution {
    pub fn discount_prices(sentence: String, discount: i32) -> String {
        let mut new_str = String::new();
        
        for (i, word) in sentence.split_whitespace().enumerate(){
            if i != 0{
                new_str.push(' ');
            } 
            if word.len()> 0{
                if word.as_bytes()[0] == b'$'{
                    if (&word[1..]).chars().all(|c|{c >= '0' && c<= '9'}){
                        if let Ok(num) = &word[1..].parse::<f64>(){
                            println!("{:?}", (&word[1..], num));
                            let n = num  * ((100 - discount) as f64 / 100_f64);

                            new_str.push_str(&format!("${:.2}", n));
                            continue;
                        }
                    }
                }
            }

            new_str.push_str(word);
        }
        
        new_str
    }
}