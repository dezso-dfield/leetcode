// O(n)

/**
 * @param {string} s
 * @return {boolean}
 */
var isPalindrome = function(s) {
    const sanitized = s.toLowerCase().replace(/[^a-z0-9]/g, '');
    const reversed = sanitized.split('').reverse().join('');
    return sanitized === reversed;
};
