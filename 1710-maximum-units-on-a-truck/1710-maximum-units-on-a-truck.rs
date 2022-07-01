impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        let mut res = 0;
        box_types.sort_by_key(|x| -x[1]);
        for p in box_types {
            let (boxes, units) = (p[0], p[1]);
            let x = truck_size.min(boxes);
            truck_size -= x;
            res += x * units;
        }
        res
    }
}