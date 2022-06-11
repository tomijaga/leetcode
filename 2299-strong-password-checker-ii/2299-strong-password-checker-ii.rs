impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        if password.len() < 8{
            return false;
        }
        
        let (mut uppercase, mut lowercase, mut digit, mut special) = (false, false, false, false);
        
        let special_chars = "!@#$%^&*()-+";
        let mut prev = ' ';
        
        for c in password.chars(){
            if c.is_uppercase(){
                uppercase = true;
                
            }else if c.is_lowercase(){
                lowercase = true;
                
            }else if c.is_digit(10){
                digit = true;
                
            }else if special_chars.contains(&c.to_string()){
                special  = true;
            }
            
            if prev == c{
                return false;
            }
            
            prev = c;
        }
        
        return uppercase && lowercase && digit && special;
    }
}