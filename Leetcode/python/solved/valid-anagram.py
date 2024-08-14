class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        def str2dict(st: str) -> dict:
            map = {}
            for c in st:
                if c in map:
                    map[c] += 1
                else:
                    map[c] = 1
            return map
        map1 = str2dict(s)
        map2 = str2dict(t)
        return map1 == map2
