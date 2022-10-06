use std::collections::{HashMap, HashSet, VecDeque};

type Tweet = (i32, i32);

#[derive(Default)]
struct Twitter {
    tweets: HashMap<i32, VecDeque<Tweet>>,
    followers: HashMap<i32, HashSet<i32>>,
    time: i32, // assuming the tweet rate is 1 
}

impl Twitter {

    fn new() -> Self {
        Default::default()
    }
    
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.followers.entry(user_id)
            .or_insert(HashSet::new());
        
        let tweets = self.tweets.entry(user_id)
            .or_insert(VecDeque::new());
        
        tweets.push_back((self.time, tweet_id));
        
        if tweets.len() > 10{
            tweets.pop_front();
        }
        self.time+=1;
    }
    
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut feed = vec![];
        let mut reference = vec![];
        
        if let Some(ppl_user_follows) = self.followers.get(&user_id){
            for followee_id in ppl_user_follows.into_iter().chain([user_id].into_iter()){
                if let Some(tweets) = self.tweets.get(&followee_id){
                    reference.push((tweets.len(), tweets));
                }
            }
        }
        
        while feed.len() < 10{
            if let Some((i, tuple)) = reference.iter_mut()
                .enumerate()
                .max_by(|(_, a), (_, b)| {
                    a.1.get(a.0 - 1).cmp(&(b.1.get(b.0 - 1)))
                })
            {
                if tuple.0 == 0{
                    break;
                }
                
                tuple.0 -=1;
                    
                let (pos, tweets) = *tuple;
                
                feed.push(tweets[pos].1);
                    
            }else{
                break;
            }
        }
        
        feed
    }
    
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.followers.entry(follower_id)
            .or_insert(HashSet::new())
            .insert(followee_id);
    }
    
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.followers.entry(follower_id)
            .or_insert(HashSet::new())
            .remove(&followee_id);
    }
}
