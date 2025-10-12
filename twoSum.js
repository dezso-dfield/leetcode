// O(n2) Solution (brute force)

/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
var twoSum = function(nums, target) {
    let result = []
    for (let i = 0;i < nums.length;i++) {
        for (let j = i+1;j < nums.length;j++) {
            if (nums[i] + nums[j] == target) {
                result.push(i);
                result.push(j);
            }
        }
    }
    return result;
};

// O(n) solution

/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
var twoSum = function(nums, target) {
    let storage = {}
    let output = []
    for (let i = 0; i < nums.length; i++) {
        const currentNum = nums[i];
        const complement = target - currentNum
        if (complement in storage) {
            output.push(storage[complement])
            output.push(i)
            return output
        } else {
            number = nums[i]
            storage[number] = i;
        }
    }
    console.log(storage)
    return []
};
