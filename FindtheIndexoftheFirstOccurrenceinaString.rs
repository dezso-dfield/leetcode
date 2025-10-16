impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        if needle.len() > haystack.len() {
            return -1;
        }

        let h = haystack.as_bytes();
        let n = needle.as_bytes();

        for i in 0..=h.len() - n.len() {
            if &h[i..i + n.len()] == n {
                return i as i32;
            }
        }
        -1
    }
}
