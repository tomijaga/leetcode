use std::collections::{HashMap, BinaryHeap};

impl Solution {
    pub fn most_popular_creator(creators: Vec<String>, ids: Vec<String>, views: Vec<i32>) -> Vec<Vec<String>> {
        
        let mut view_map = HashMap::new();
        let mut id_map = HashMap::new();
        
        let mut max_sum = 0;
        
        let mut a = creators.into_iter();
        let mut b = ids.into_iter();
        let mut c = views.into_iter();
        
        for ((creator, id), views) in a.zip(b).zip(c){
            
            let new_movie = (views, id);
            let movie = id_map.entry(creator.clone()).or_insert(new_movie.clone());
            
            if new_movie.0 > movie.0{
                *movie = new_movie;
            }else if movie.0 == new_movie.0 && new_movie.1 < movie.1{
                *movie = new_movie;
            }
            
            let sum = view_map.entry(creator).or_insert(0_i64);
            *sum = *sum + views as i64;
            
            max_sum = max_sum.max(*sum);
        }
        
        let mut res = vec![];
        
        for (creator, views) in view_map{
            if views == max_sum{
                let id = id_map.remove(&creator).unwrap().1;
                res.push(vec![creator, id]);
            }
        }
        
        res
    }
}