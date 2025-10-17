impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() {
            return vec![];
        }

        let mut result: Vec<String> = Vec::new();
        let mut i = 0;
        while i < nums.len() {
            let start = nums[i];
            while i + 1 < nums.len() && nums[i + 1] == nums[i] + 1 {
                i += 1;
            }
            let end = nums[i];
            if start == end {
                result.push(start.to_string());
            } else {
                result.push(format!("{}->{}", start, end));
            }
            i += 1;
        }

        result
    }
}
