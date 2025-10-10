// O(n) solution

/**
 * @param {number} x
 * @return {boolean}
 */
var isPalindrome = function(x) {
    x = String(x)
    let result = []
    for (let i = x.length-1; i >= 0; i--) {
        result.push(x[i])
    }
    result = result.join("")
    if (result == x) {
        return true
    } else {
        return false;
    }
    console.log(result)
};

// Without string conversion and faster runtime

/**
 * @param {number} x
 * @return {boolean}
 */
var isPalindrome = function(x) {
if (x < 0 || (x % 10 === 0 && x !== 0)) {
        return false;
    }

    let revertedNumber = 0;
    let originalX = x;

    while (originalX > revertedNumber) {
        revertedNumber = revertedNumber * 10 + originalX % 10;
        originalX = Math.floor(originalX / 10);
    }
    return originalX === revertedNumber || originalX === Math.floor(revertedNumber / 10);
};
