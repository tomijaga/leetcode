use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut colors: Vec<i32> = vec![-1; graph.len()];
        
        (0..graph.len()).all(|i|{
            if colors[i] == -1{
                dfs(&graph, &mut colors, i, 1)
            }else{
                true
            }
        })
    }
}

pub fn dfs(
    graph: &Vec<Vec<i32>>, 
    colors: &mut Vec<i32>, 
    i: usize,
    color: i32
) -> bool {
    colors[i] = color;
    
    let mut res = true;
    for &j in graph[i].iter(){
        let j = j as usize;
        if colors[j] != -1{
            if colors[j] == color{
                return false;
            }
        }else{
            res &= dfs(graph, colors, j, !color);
        }
    } 
    
    res
}