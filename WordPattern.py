class Solution:
    def wordPattern(self, pattern: str, s: str) -> bool:
        arr = s.split(" ")
        if len(pattern) != len(arr):
            return False
        l = 0
        mapp = {}
        mapper = {}
        while l < len(pattern):
            if pattern[l] not in mapp and arr[l] not in mapper:
                mapp[pattern[l]] = arr[l]
                mapper[arr[l]] = pattern[l]
            if pattern[l] in mapp and mapp[pattern[l]] != arr[l]:
                return False
            if arr[l] in mapper and mapper[arr[l]] != pattern[l]:
                return False
            l += 1
        return True
