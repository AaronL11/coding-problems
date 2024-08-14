class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        return reduce(lambda x,acc: acc^x, nums)
