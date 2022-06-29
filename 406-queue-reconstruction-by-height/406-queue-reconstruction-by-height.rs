
impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_unstable_by(|a, b|{
            if b[0] == a[0]{
                (a[1]).cmp(&b[1])
            }else{
                (b[0]).cmp(&a[0])
            }
        });
        
        // println!("{:?}", &people);
        let mut q = Vec::with_capacity(people.len());
        
        for v in people{
            q.insert(v[1] as usize, v);
        }
        
        q
    }
}