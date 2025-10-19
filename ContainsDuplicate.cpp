class Solution {
public:
    bool containsDuplicate(std::vector<int>& nums) {
        std::unordered_set<int> seen_numbers;

        for (int n : nums) {
            if (seen_numbers.count(n)) {
                return true;
            }
            seen_numbers.insert(n);
        }
        return false;
    }
};
