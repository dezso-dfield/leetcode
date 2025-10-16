impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l:i32 = 0;
        let mut r:i32 = nums.len() as i32;
        while l < r {
            let mid:i32 = l + (r-l) / 2;
            if nums[mid as usize] == target {
                return mid;
            }
            else if nums[mid as usize] < target {
                l = mid + 1;
            }
            else {
                r = mid;
            }
        }
        l
    }
}
