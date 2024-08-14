class Solution:
    def firstUniqChar(self, s: str) -> int:
        map = {}
        for c in s:
            if c in map:
                map[c] += 1
            else:
                map[c] = 1
        for i,c in enumerate(s):
            if map[c] == 1:
                return i
        return -1
