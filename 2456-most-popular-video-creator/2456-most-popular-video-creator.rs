use std::collections::{HashMap, BinaryHeap};

impl Solution {
    pub fn most_popular_creator(creators: Vec<String>, ids: Vec<String>, views: Vec<i32>) -> Vec<Vec<String>> {
        
        let mut map = HashMap::new();
        
        let mut max_sum = 0;
        
        let mut a = creators.into_iter();
        let mut b = ids.into_iter();
        let mut c = views.into_iter();
        
        for ((creator, id), views) in a.zip(b).zip(c){
            
            let new_movie = (views, id);
            let entry = map.entry(creator.clone()).or_insert((0, new_movie.clone()));
            let (mut total_views, movie) = entry;
            
            if new_movie.0 > movie.0{
                entry.1 = new_movie;
            }else if movie.0 == new_movie.0 && new_movie.1 < movie.1{
                entry.1 = new_movie;
            }
            
            total_views = total_views + views as i64;
            entry.0 = total_views;
            max_sum = max_sum.max(total_views);
        }
        
        let mut res = vec![];
        
        for (creator, (total_views, movie)) in map{
            if total_views == max_sum{
                res.push(vec![creator, movie.1]);
            }
        }
        
        res
    }
}