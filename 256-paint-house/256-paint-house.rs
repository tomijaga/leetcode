impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (costs.len(), costs[0].len());
        
        let mut cache = vec![vec![-1; n]; m];
        
        dp(&costs, &mut cache, 0, 0);
        dp(&costs, &mut cache, 0, 1);
        dp(&costs, &mut cache, 0, 2);
        // println!("{:?}", &cache);
        
        *cache[0].iter().min().unwrap()
    }
}

pub fn dp(costs: &Vec<Vec<i32>>, cache: &mut Vec<Vec<i32>>, i: usize, color: usize){
 let min_child = (0..=2).map(|c|{
        if c != color{
            if i + 1 == cache.len(){
                0
            }else{
                if cache[i+1][c] == -1{
                    dp(costs, cache, i + 1, c);
                }
                cache[i+1][c]
            }
        }else{
            i32::MAX
        }
    })
    .min()
    .unwrap();
        
    cache[i][color] = costs[i][color] + min_child;
        
}