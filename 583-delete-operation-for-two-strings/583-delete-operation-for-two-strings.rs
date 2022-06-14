impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let n = word1.len();
        let mut a = (0..=n).collect::<Vec<_>>();
        let mut b = Vec::with_capacity(n + 1);
        for (i, &c) in word2.iter().enumerate() {
            b.clear();
            let mut prev = i + 1;
            b.push(prev);
            for (j, &d) in word1.iter().enumerate() {
                prev = if c == d { a[j] } else { prev.min(a[j + 1]) + 1 };
                b.push(prev);
            }
            std::mem::swap(&mut a, &mut b);
        }
        a[n] as i32
    }
}