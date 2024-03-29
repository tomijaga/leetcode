#[derive(Default)]
struct StockSpanner {
            // (val, pos)
    stack: Vec<(i32, i32)>,
    index : i32
}

impl StockSpanner {

    fn new() -> Self {
        Default::default()
    }
    
    fn next(&mut self, price: i32) -> i32 {
        self.index += 1;
        
        let mut span = 1;
        
        while let Some(&(prev_val, prev_index)) = self.stack.last(){
            if prev_val <= price{
                self.stack.pop();
            }else{
                span = self.index - prev_index;
                break;
            }
        }
        
        if self.stack.is_empty(){
            span = self.index;
        };
        
        self.stack.push((price, self.index));
        
        span
    }
}