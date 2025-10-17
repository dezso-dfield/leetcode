impl Solution {
    pub fn missing_number(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        for (i, n) in nums.iter().enumerate() {
            if *n != i as i32 {
                return i as i32;
            }
        }
        nums.len() as i32
    }
}
