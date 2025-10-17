use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut char_counts = HashMap::new();
        for ch in s.chars() {
            let count = char_counts.entry(ch).or_insert(0);
            *count += 1;
        }
        for ch in t.chars() {
            let count = char_counts.entry(ch).or_insert(0);
            *count -= 1;
        }
        return char_counts.values().all(|&count| count == 0);
    }
}
