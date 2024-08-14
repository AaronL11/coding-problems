class Solution:
    def majorityElement(self, nums: List[int]) -> int:
        n = len(nums)

        dict = {}
        for num in nums:
            if num in dict:
                dict[num] += 1
            else:
                dict[num] = 1
        for k,v in dict.items():
            if v > n//2:
                return k
