use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut left_set = HashMap::new();
        let mut visited = HashSet::new();
        
        (0..graph.len()).all(|i|{
            if !left_set.contains_key(&(i as i32)){
                dfs(&graph, &mut visited, &mut left_set, i as i32, true)
            }else{
                true
            }
        })
    }
}

pub fn dfs(
    graph: &Vec<Vec<i32>>, 
    visited: &mut HashSet<i32>, 
    left_set: &mut HashMap<i32, bool>, 
    i: i32,
    is_left: bool
) -> bool {
    // visited.insert(i as i32);
    left_set.insert(i, is_left);
    
    let mut res = true;
    for &j in graph[i as usize].iter(){
        if let Some(&pos) = left_set.get(&j){
            if pos == is_left{
                return false;
            }
        }else{
            res &= dfs(graph, visited, left_set, j, !is_left);
        }
    } 
    
    res
}