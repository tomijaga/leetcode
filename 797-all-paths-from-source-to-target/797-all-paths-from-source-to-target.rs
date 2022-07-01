use std::collections::VecDeque;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        
        dfs(&graph, &mut res, vec![0], 0);
        
        res
    }
}

pub fn dfs(graph: &Vec<Vec<i32>>, res: &mut Vec<Vec<i32>>, nums: Vec<i32>, i: usize){
    
    let last = graph.len() - 1;
    let l_i32 = last as i32;
    
    if nums.last().unwrap() == &l_i32 {
        if nums.len() > 0{
            res.push(nums);
        }
        return;
    }
    
    let vertices = &graph[i];
    
    for &v in vertices{
        let mut new_nums = nums.clone();
        new_nums.push(v);
        
        dfs(graph, res, new_nums, v as usize);
    }
    
}