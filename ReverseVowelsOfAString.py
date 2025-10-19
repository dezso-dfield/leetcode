class Solution:
    def reverseVowels(self, s: str) -> str:
        s_list = list(s)
        l, r = 0, len(s)-1
        vowels = set('aeiouAEIOU')
        while l < r:
            if s_list[l] not in vowels:
                l += 1
            if s_list[r] not in vowels:
                r -= 1
            if s_list[l] in vowels and s_list[r] in vowels:
                temp = s_list[l]
                s_list[l] = s_list[r]
                s_list[r] = temp
                l += 1
                r -= 1
        s = "".join(s_list)
        return s
