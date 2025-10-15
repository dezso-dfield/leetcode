class Solution {
public:
    bool isPerfectSquare(int num) {
        long l = 1;
        long r = num;
        while (l <= r) {
            long current = l + (r - l) / 2;
            long square = current * current;
            if (square == num) {
                return true;
            }
            else if (square > num) {
                r = current-1;
            } else if (square < num) {
                l = current+1;
            }
        }
        return false;
    }
};
