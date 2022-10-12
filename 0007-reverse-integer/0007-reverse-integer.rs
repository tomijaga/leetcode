impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut negative = x < 0;
        if negative{
            x *= -1;
        }
        
        let len = ((x as f32).log10() as i32 + 1) as u32;
        
        let mut i = 0;
        let mut reversed = 0;
        
        while x > 0{
            let n = (x % 10) as i64;
            x /= 10;
            reversed += n * 10_i64.pow(len - 1 - i);
            i+=1;
        }
        
        
        let res = if negative{
            -reversed
        }else{
            reversed
        };
        
        if res > i32::MAX as i64 || res < i32::MIN as i64{
            0
        }else{
            res as i32
        }
        
    }
}