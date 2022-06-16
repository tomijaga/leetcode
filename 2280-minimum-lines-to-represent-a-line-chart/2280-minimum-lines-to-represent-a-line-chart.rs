impl Solution {
    pub fn minimum_lines(mut stock_prices: Vec<Vec<i32>>) -> i32 {
        if stock_prices.len() ==1{
            return 0;
        }
        stock_prices.sort_unstable_by(|a, b|{a[0].cmp(&b[0])});
        
        let mut lines = 1;
        for i in 1..stock_prices.len() -1{
            // println!("i:{:?}", (i, i+1));
            if gradient(&stock_prices[i-1], &stock_prices[i], &stock_prices[i+1]){
                lines +=1;
            }
        }
        
        lines
    }
}

pub fn gradient(a: &Vec<i32>, b: &Vec<i32>, c: &Vec<i32>)-> bool {
    (b[1]-a[1]) * (c[0]-b[0]) != (c[1]-b[1]) * (b[0]-a[0])
}