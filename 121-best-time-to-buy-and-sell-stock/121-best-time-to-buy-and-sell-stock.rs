impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        
        let mut prev_max = i32::MAX;
        let mut buy_price = i32::MAX;
        
        for price in prices{
            
            prev_max = prev_max.max(price);
            let profit = prev_max - buy_price;
            max_profit = max_profit.max(profit);
            
            if price < buy_price{
                buy_price = price;
                prev_max = price;
            }
        }
        
        max_profit
    }
}