class Solution:
    def titleToNumber(self, columnTitle: str) -> int:
        def map(char: str) -> int:
            return ord(char) - 64
        def recurse(columnTitle: str) -> int:
            if len(columnTitle) == 0:
                return 0
            elif len(columnTitle) == 1:
                return map(columnTitle[-1])
            else:
                return map(columnTitle[-1]) + 26*recurse(columnTitle[:-1])
        return recurse(columnTitle)
        
