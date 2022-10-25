impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.into_iter().collect::<String>() == word2.into_iter().collect::<String>()
    }
}