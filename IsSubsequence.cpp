class Solution {
public:
    bool isSubsequence(string s, string t) {
        if (s == "") {
            return true;
        }
        short l = 0;
        short r = 0;
        while (r < t.size()) {
            if (t[r] == s[l]) {
                l++;
            }
            r++;
            if (l == s.size()) {
                return true;
            }
        }
        return false;
    }
};
