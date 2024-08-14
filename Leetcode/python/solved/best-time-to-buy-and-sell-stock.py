class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        mx = 0
        buy = prices[0]
        for p in prices[1:]:
            if p < buy:
                buy = p
            else:
                mx = max(p-buy, mx)
        return mx
        
