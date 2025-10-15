#include <string>
#include <unordered_map>

class Solution {
public:
    char findTheDifference(std::string s, std::string t) {
        std::unordered_map<char, int> charCount;
        for (char c : s) {
            charCount[c]++;
        }
        for (char c : t) {
            if (charCount[c] == 0) {
                return c;
            }
            charCount[c]--;
        }
        return ' ';
    }
};
