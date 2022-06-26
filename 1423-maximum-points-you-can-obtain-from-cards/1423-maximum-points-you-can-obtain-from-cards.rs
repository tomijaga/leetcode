impl Solution {
    pub fn max_score(mut card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let len = card_points.len() ;
        if k == card_points.len(){
            return card_points.into_iter().sum::<i32>();
        }
        
        let mut total = 0;
        
        for card in card_points.iter_mut(){
            let tmp = *card;
            *card = total;
            total += tmp;
        }
        
        card_points.push(total);
        println!("{:?}", &card_points);
        
        let window = len - k;
        let mut max_points = 0;
        
        for i in 0..=k{
            let sum = card_points[i + window] - card_points[i];
            max_points = max_points.max(total - sum);
        }
        
        max_points
    }
}