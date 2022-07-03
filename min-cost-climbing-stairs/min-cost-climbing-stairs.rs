impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        if cost.len() == 1{
            return cost[0];
        }
        
        let mut cache = vec![0; cost.len()];
        cache[0] = cost[0];
        cache[1] = cost[1];
        
        for i in 2..cost.len(){
            cache[i] = cache[i- 1].min(cache[i-2])  + cost[i];
        }
        
        cache[cost.len() - 1].min(cache[cost.len() - 2])
    }
}