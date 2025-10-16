impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut insert_pos:i32 = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[insert_pos as usize] = nums[i];
                insert_pos += 1;
            }
        }
        while insert_pos < nums.len() as i32 {
            nums[insert_pos as usize] = 0;
            insert_pos += 1;
        }
    }
}
