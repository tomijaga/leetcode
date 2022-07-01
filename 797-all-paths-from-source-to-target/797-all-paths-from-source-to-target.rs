use std::collections::VecDeque;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut nums = vec![];
        
        dfs(&graph, &mut res, &mut nums, 0);
        
        res
    }
}

pub fn dfs(graph: &Vec<Vec<i32>>, res: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>, n: i32){
    
    let last = graph.len() - 1;
    
    nums.push(n);
    
    if n == last as i32 {
        if nums.len() > 0{
            res.push(nums.clone());
            nums.pop();
        }
        
        return;
    }
    
    let vertices = &graph[n as usize];
    
    for &v in vertices{
        dfs(graph, res, nums, v);
    }
    
    nums.pop();
    
}