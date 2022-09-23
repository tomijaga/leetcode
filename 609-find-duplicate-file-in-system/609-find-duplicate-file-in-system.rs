use std::collections::BTreeMap;
use std::fmt::Write as FmtWrite;

impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut map = BTreeMap::new();
        
        for path in paths{
            
            let mut path_iter = path.split_whitespace();
            let dir = path_iter.next().unwrap();
            
            for file in path_iter{
                
                let mut file_iter = file.split("(");
                let filename = file_iter.next().unwrap();
                let tmp = file_iter.next().unwrap();
                let content = String::from(&tmp[..tmp.len() - 1]);
                
                map.entry(content)
                    .or_insert(vec![])
                    .push(format!("{}/{}", dir, filename));
            }
        }
        
        map.into_values()
            .filter(|v| v.len() > 1)
            .collect::<Vec<Vec<String>>>()
    }
}